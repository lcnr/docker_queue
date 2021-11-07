use crate::helpers::spawn_app;

#[tokio::test]
async fn queue_container_adds_to_queue() {
    // Arrange
    let mut app = spawn_app().await;
    let name = "a_queued_container";

    // Act
    app.client.queue_container(name).await.unwrap();
    let output = app.get_client_output();
    println!("{}", output);

    // Assert
    assert!(output.contains(name))
}

#[tokio::test]
async fn queue_container_runs_if_no_running_containers() {
    // // Arrange
    // let app = spawn_app().await;
    // let mut buffer = Vec::new();
    // let name = "a_queued_container";

    // // Act
    // queue_container(name, app.port, &mut buffer).await.unwrap();
    // let output = String::from_utf8(buffer).expect("Failed to get string from buffer.");
    // println!("{}", output);

    // // Assert
    // assert!(output.contains(name))
}
