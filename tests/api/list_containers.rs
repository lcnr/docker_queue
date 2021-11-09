use crate::helpers::{rm_sleeping_container, run_sleeping_container, spawn_app};

#[tokio::test]
async fn list_containers_contains_running_containers() {
    // Arrange
    let mut app = spawn_app().await;
    let container_id = run_sleeping_container(120).await.unwrap();

    // Act
    app.client.list_containers().await.unwrap();
    let output = app.get_client_output();
    println!("{}", output);

    // Clean
    rm_sleeping_container(&container_id).await.unwrap();

    // Assert
    assert!(output.contains(&container_id));
}

#[tokio::test]
async fn list_containers_contains_queued_containers() {
    // Arrange
    let mut app = spawn_app().await;
    let name = "docker run -d some_image";
    app.client.queue_container(name, true).await.unwrap();
    println!("{}", app.get_client_output());

    // Act
    app.client.list_containers().await.unwrap();
    let output = app.get_client_output();
    println!("{}", output);

    // Assert
    assert!(output.contains(name));
}
