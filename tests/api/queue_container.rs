use crate::helpers::spawn_app;

#[tokio::test]
async fn queue_container_adds_to_queue() {
    // Arrange
    let mut app = spawn_app().await;
    let command = "docker run -d some_image";

    // Act
    app.client.queue_container(command, true).await.unwrap();
    let output = app.get_client_output();
    println!("{}", output);

    // Assert
    assert!(output.contains("added to queue"));
}

#[tokio::test]
async fn queue_container_runs_if_no_running_containers() {
    // Arrange
    let mut app = spawn_app().await;
    let command = "docker run -d --rm alpine sleep 2";
    app.client.queue_container(command, false).await.unwrap();
    println!("{}", app.get_client_output());

    // Act
    app.client.list_containers().await.unwrap();
    let output = app.get_client_output();
    println!("{}", output);

    // Assert
    let line = output
        .lines()
        .filter(|line| line.contains(command))
        .collect::<String>();
    assert!(line.contains("Running"));
}
