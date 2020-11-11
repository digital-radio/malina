use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};

use dbus::blocking::Connection;
use dbus_crossroads::{Context, Crossroads, MethodErr};
use serde::{Deserialize, Serialize};
use serde_json::to_string as json_encode;

use shell::Shell;

mod shell;

#[derive(Serialize, Deserialize)]
struct WifiOnData {
    ssid: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct WifiOnResponse  {
    code: String,
}

pub struct WifiOnHandler {
    shell: Arc<Mutex<Shell>>,
}

impl WifiOnHandler {
    pub fn new(shell: Arc<Mutex<Shell>>) -> WifiOnHandler {
        WifiOnHandler { shell }
    }

    pub fn handle(&self, _ctx: &mut Context, body: String) -> Result<(String,), MethodErr> {
        let parsed_data: WifiOnData = match serde_json::from_str(body.as_str()) {
            Ok(d) => d,
            Err(_) => return Err(("pl.digitalradio.JsonDecodeError", "Could not decode args").into()),
        };

        println!("Connect to Wi-Fi [{}]", parsed_data.ssid);
        let shell = self.shell.lock().unwrap();
        shell.connect_wifi(&parsed_data.ssid, &parsed_data.password);

        let response = WifiOnResponse {
            code: "ok".to_owned(),
        };
        let response_data = match json_encode(&response) {
            Ok(d) => d,
            Err(_) => return Err(("pl.digitalradio.JsonEncodeError", "Could not encode response").into()),
        };
        Ok((response_data,))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let shell = Arc::new(Mutex::new(Shell::new(args[1].to_owned())));

    let c = Connection::new_session()?;
    c.request_name("pl.digitalradio", false, true, false)?;

    let mut cr = Crossroads::new();

    let iface_token = cr.register("pl.digitalradio", |b| {
        b.method("wifi_on", ("data", ), ("response", ), |_ctx: &mut Context, _data: &mut WifiOnHandler, (body, ): (String, )| {
            _data.handle(_ctx, body)
        });
    });


    let wifi_on_handler = WifiOnHandler::new(Arc::clone(&shell));
    cr.insert("/malina", &[iface_token], wifi_on_handler);
    cr.serve(&c)?;

    unreachable!()
}
