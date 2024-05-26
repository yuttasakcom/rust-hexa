use hexa::app::server::Server;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let app = Server::new();
    app.run().await;
}
