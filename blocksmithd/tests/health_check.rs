use api;
mod tests {
    use super::*;
    use std::net::TcpListener;

    fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();

        let server = api::run(listener).expect("failed to bind address");

        let _ = tokio::spawn(server);

        format!("http://localhost:{}", port)
    }

    #[tokio::test]
    async fn health_check_works() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        let response = client
            .get(address)
            .send()
            .await
            .expect("Failed to execute request");

        println!("response: {:?}", response);
        assert!(response.status().is_success());
    }
}
