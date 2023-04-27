use std::net::TcpListener;

fn spawn_app() -> String {
    let address = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", address)).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter_rs::run_server(listener).expect("Failed to bind address");

    tokio::spawn(server);

    format!("http://{}:{}", address, port)
}

#[tokio::test]
async fn health_check_endpoint_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health-check", &address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
