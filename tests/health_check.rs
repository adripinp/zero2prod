//use zero2prod::main;

#[tokio::test]
async fn health_check_works() {
    spawn_app()
}

//async fn health_check_works() {
//    spawn_app().await.expect("Failed to spawn our app.");
//    let client = reqwest::Client::new();

//    let response = client
//        .get("http://127.0.0.1:8080/health")
//        .send()
//        .await
//        .expect("Failed to send request.");
        
//    assert!(response.status().is_success());
//    assert_eq!(Some(0), response.content_length());

//}


fn spawn_app() {
    let server= zero2prod::run().expect("Failed to bind asddress");

    let _ = tokio::spawn(server);
}
