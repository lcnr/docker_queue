use crate::helpers::{run_sleeping_container, rm_sleeping_container, spawn_app};

#[tokio::test]
async fn list_containers_works() {
    // Arrange
    let app = spawn_app().await;
	let container_id = run_sleeping_container(120).await.unwrap();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://127.0.0.1:{}/list_containers", app.port))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    println!("{:#?}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

	// Clean
	rm_sleeping_container(container_id).await.unwrap();
}
