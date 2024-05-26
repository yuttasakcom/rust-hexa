use axum::{routing::get, Router};

pub struct AxumAdapter {
    addr: String,
}

impl AxumAdapter {
    pub fn new(addr: String) -> AxumAdapter {
        AxumAdapter { addr }
    }

    pub async fn run(&self) {
        let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));
        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
