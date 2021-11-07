use crate::helpers::{rm_sleeping_container, run_sleeping_container, spawn_app};
use docker_queue::client::{list_containers, queue_container};

#[tokio::test]
async fn list_containers_contains_running_containers() {
    // Arrange
    let app = spawn_app().await;
    let container_id = run_sleeping_container(120).await.unwrap();
    let mut buffer = Vec::new();

    // Act
    list_containers(app.port, &mut buffer).await.unwrap();
    let output = String::from_utf8(buffer).expect("Failed to get string from buffer.");
    println!("{}", output);

    // Clean
    rm_sleeping_container(&container_id).await.unwrap();

    // Assert
    assert!(output.contains(&container_id));
}

#[tokio::test]
async fn list_containers_contains_queued_containers() {
    // Arrange
    let app = spawn_app().await;
    let name = "a_queued_container";
    let mut buffer = Vec::new();
    queue_container(name, app.port, &mut buffer).await.unwrap();
    let output = String::from_utf8(buffer).expect("Failed to get string from buffer.");
    println!("{}", output);

    // Act
    let mut buffer = Vec::new();
    list_containers(app.port, &mut buffer).await.unwrap();
    let output = String::from_utf8(buffer).expect("Failed to get string from buffer.");
    println!("{}", output);

    // Assert
    assert!(output.contains(name));
}
