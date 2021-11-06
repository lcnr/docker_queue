use docker_queue::domain::Container;

use crate::helpers::{rm_sleeping_container, run_sleeping_container, spawn_app};

#[tokio::test]
async fn list_containers_contains_running_containers() {
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
    rm_sleeping_container(&container_id).await.unwrap();

    // Assert
    assert!(response.status().is_success());
    let n = response
        .json::<Vec<Container>>()
        .await
        .expect("Failed to deserialize body.")
        .into_iter()
        .filter(|container| match container {
            Container::Running(x) => x.id.as_ref().map(|id| id == &container_id).unwrap_or(false),
            _ => false,
        })
        .count();
    assert_eq!(n, 1);
}
