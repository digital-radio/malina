use core::result::Result::{Err, Ok};
use std::sync::{Arc, Mutex};

use dbus_crossroads::MethodErr;
use serde::{Deserialize, Serialize};
use serde_json::ser::to_string as json_encode;

use crate::shell::Shell;

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

    pub fn handle(&self, body: String) -> Result<(String,), MethodErr> {
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

mod test {
    use crate::wifi_on_handler::WifiOnHandler;
    use std::sync::{Arc, Mutex};
    use crate::shell::Shell;

    #[test]
    fn returns_error_on_invalid_body() {
        // given
        let shell = Arc::new(Mutex::new(Shell::new("/test".to_owned())));
        let handler = WifiOnHandler::new(Arc::clone(&shell));

        // when
        let result = handler.handle("example".to_owned());

        // then
        assert_eq!(
            Err(("pl.digitalradio.JsonDecodeError", "Could not decode args").into()),
            result
        );
    }
}
