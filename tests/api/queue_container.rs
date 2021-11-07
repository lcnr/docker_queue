use crate::helpers::spawn_app;
use docker_queue::client::queue_container;

#[tokio::test]
async fn queue_container_adds_to_queue() {
    // Arrange
    let app = spawn_app().await;
    let mut buffer = Vec::new();
    let name = "a_queued_container";

    // Act
    queue_container(name, app.port, &mut buffer).await.unwrap();
    let output = String::from_utf8(buffer).expect("Failed to get string from buffer.");
    println!("{}", output);

    // Assert
    assert!(output.contains(name))
}
