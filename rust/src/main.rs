use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};

use dbus::blocking::Connection;
use dbus_crossroads::{Context, Crossroads};

use shell::Shell;

use crate::wifi_on_handler::WifiOnHandler;

mod shell;
mod wifi_on_handler;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let shell = Arc::new(Mutex::new(Shell::new(args[1].to_owned())));

    let c = Connection::new_session()?;
    c.request_name("pl.digitalradio", false, true, false)?;

    let mut cr = Crossroads::new();
    register_wifi_on_handler(&mut cr, &shell);

    cr.serve(&c)?;
    unreachable!()
}

fn register_wifi_on_handler(cr: &mut Crossroads, shell: &Arc<Mutex<Shell>>) {
    let iface_token = cr.register("pl.digitalradio", |b| {
        b.method(
            "wifi_on",
            ("data", ),
            ("response", ),
            |_ctx: &mut Context, data: &mut WifiOnHandler, (body, ): (String, )| data.handle(body));
    });

    let wifi_on_handler = WifiOnHandler::new(Arc::clone(&shell));
    cr.insert("/malina", &[iface_token], wifi_on_handler);
}
