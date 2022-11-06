#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: std::env::var("PORT")
                .map_err(|_| "Failed to get port")
                .and_then(|port| port.parse::<u16>().map_err(|_| "Failed to parse port"))
                .unwrap_or(80),
        }
    }
}
