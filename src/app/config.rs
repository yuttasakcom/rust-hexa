pub struct Config {
    host: String,
    port: u32,
}

impl Config {
    pub fn new(host: String, port: u32) -> Config {
        Config { host, port }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
