use crate::{adapter::axum::AxumAdapter, app::config::Config};

pub struct Server {
    config: Config,
    app: AxumAdapter,
}

impl Server {
    pub fn new() -> Server {
        let config = Config::new("0.0.0.0".to_string(), 8080);
        let app = AxumAdapter::new(config.addr());
        Server { config, app }
    }

    pub async fn run(&self) {
        println!("Server is running on {}", self.config.addr());
        self.app.run().await;
    }
}
