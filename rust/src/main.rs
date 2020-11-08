use std::error::Error;

use dbus::blocking::Connection;
use dbus_crossroads::{Crossroads, Context, MethodErr, IfaceToken};
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

fn main() -> () {
    match server() {
        Ok(_) => unreachable!(),
        Err(e) => panic!("{:?}", e),
    }
}

fn server() -> Result<(), Box<dyn Error>> {
    let c = Connection::new_session()?;
    c.request_name("pl.digitalradio", false, true, false)?;

    let cr = create_crossroad();
    cr.serve(&c)?;

    unreachable!()
}

fn create_crossroad() -> Crossroads {
    let mut cr = Crossroads::new();
    let iface_token = create_interface(&mut cr);
    cr.insert("/malina", &[iface_token], ());
    cr
}

fn create_interface(cr: &mut Crossroads) -> IfaceToken<()> {
    cr.register("pl.digitalradio", |b| {
        b.method("wifi_on", ("data", ), ("response", ), handle_wifi_on);
    })
}

fn handle_wifi_on(_ctx: &mut Context, _data: &mut (), (body,): (String,)) -> Result<(String,), MethodErr> {
    let parsed_data: WifiOnData = match serde_json::from_str(body.as_str()) {
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
}
