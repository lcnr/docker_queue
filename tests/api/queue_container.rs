use crate::helpers::spawn_app;

#[tokio::test]
async fn queue_container_runs_when_queue_is_empty() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        //todo
        "title": "Newsletter title",
        "content": {
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as HTML</p>"
        }
    });

    // Act
    let _response = client
        .post(format!("http://127.0.0.1:{}/queue_container", app.port))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    // rm_sleeping_container(&container_id).await.unwrap();

    // Assert
    // assert!(response.status().is_success());
    // let n = response
    //     .json::<Vec<Container>>()
    //     .await
    //     .expect("Failed to deserialize body.")
    //     .into_iter()
    //     .filter(|container| match container {
    //         Container::Running(x) => x.id.as_ref().map(|id| id == &container_id).unwrap_or(false),
    //         _ => false,
    //     })
    //     .count();
    // assert_eq!(n, 1);
}
