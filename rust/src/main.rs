use std::error::Error;

use dbus::blocking::Connection;
use dbus_crossroads::Crossroads;
use serde::{Deserialize, Serialize};
use serde_json::{to_string as json_encode};

#[derive(Serialize, Deserialize)]
struct WifiOnData {
    ssid: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct WifiOnResponse  {
    code: String,
}

fn main() {
    let result = server();

    match result {
        Ok(_) => println!("Server done"),
        Err(e) => panic!("Problem with running a client: {:?}", e),
    };
}

fn server() -> Result<(), Box<dyn Error>> {
    let c = Connection::new_session()?;
    c.request_name("pl.digitalradio", false, true, false)?;

    let mut cr = Crossroads::new();

    let iface_token = cr.register("pl.digitalradio", |b| {
        b.method("wifi_on", ("data",), ("response",), |_, _, (data,): (String,)| {
            let parsed_data: WifiOnData = match serde_json::from_str(data.as_str()) {
                    Ok(d) => d,
                    Err(_) => return Err(("pl.digitalradio.JsonDecodeError", "Could not decode args").into()),
            };

            println!("Connect to Wi-Fi [{}]", parsed_data.ssid);

            let response = WifiOnResponse {
                code: "ok".to_owned(),
            };
            let response_data = match json_encode(&response) {
                Ok(d) => d,
                Err(_) => return Err(("pl.digitalradio.JsonEncodeError", "Could not encode response").into()),
            };
            Ok((response_data,))
        });
    });

    cr.insert("/malina", &[iface_token], ());
    cr.serve(&c)?;

    unreachable!()
}
