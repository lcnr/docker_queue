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
    let command =
        "docker run -d --rm alpine sh -c \"sleep 5 && echo queue_container_runs_if_no_running_containers\"";

    // Act
    app.client.queue_container(command, false).await.unwrap();
    println!("{}", app.get_client_output());
    let lines = app
        .wait_for_running_container("queue_container_runs_if_no_running_containers", 10)
        .await
        .unwrap();
    println!("{:?}", lines);

    // Assert
    assert_eq!(lines.len(), 1);
}

#[tokio::test]
async fn queue_container_queues_if_already_running() {
    // Arrange
    let mut app = spawn_app().await;
    let command1 =
        "docker run -d --rm alpine sh -c \"sleep 5 && echo queue_container_queues_if_already_running1\"";
    let command2 =
        "docker run -d --rm alpine sh -c \"sleep 5 && echo queue_container_queues_if_already_running2\"";

    // Act
    app.client.queue_container(command1, false).await.unwrap();
    println!("{}", app.get_client_output());
    app.client.queue_container(command2, false).await.unwrap();
    println!("{}", app.get_client_output());
    app.wait_for_running_container("queue_container_queues_if_already_running1", 10)
        .await
        .unwrap();
    app.client.list_containers().await.unwrap();
    let output = app.get_client_output();
    println!("{}", output);

    // Assert
    let line1 = output
        .lines()
        .filter(|line| line.contains("queue_container_queues_if_already_running1"))
        .collect::<String>();
    let line2 = output
        .lines()
        .filter(|line| line.contains("queue_container_queues_if_already_running2"))
        .collect::<String>();
    assert!(line1.contains("Running"), "{:?}", line1);
    assert!(line2.contains("Queued"), "{:?}", line2);
}

#[tokio::test]
async fn queue_container_gets_removed_from_running_state_when_finish_execution() {
    // Arrange
    let mut app = spawn_app().await;
    let command =
        "docker run -d --rm alpine sh -c \"sleep 5 && echo queue_container_gets_removed_from_running_state_when_finish_execution\"";

    // Act
    app.client.queue_container(command, false).await.unwrap();
    println!("{}", app.get_client_output());
    app.wait_for_running_container(
        "queue_container_gets_removed_from_running_state_when_finish_execution",
        10,
    )
    .await
    .unwrap();
    // app.client.list_containers().await.unwrap();
    // let output = app.get_client_output();
    // println!("{}", output);

    // Assert
    // let line1 = output
    //     .lines()
    //     .filter(|line| line.contains("queue_container_queues_if_already_running1"))
    //     .collect::<String>();
    // let line2 = output
    //     .lines()
    //     .filter(|line| line.contains("queue_container_queues_if_already_running2"))
    //     .collect::<String>();
    // assert!(line1.contains("Running"), "{:?}", line1);
    // assert!(line2.contains("Queued"), "{:?}", line2);
}
