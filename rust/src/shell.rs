use std::path::Path;
use std::process::{Command, Output};

pub struct Shell {
    path: String,
}

impl Shell {
    pub fn new(path: String) -> Shell {
        Shell { path }
    }

    pub fn command(&self, program: &str) -> Command {
        let path = Path::new(self.path.as_str()).with_file_name(program);
        Command::new(path)
    }

    pub fn connect_wifi(&self, ssid: &str, password: &str) -> Output {
        self.command("wifi_start.sh")
            .arg(ssid)
            .arg(password)
            .output()
            .expect("failed to execute process")
    }
}
