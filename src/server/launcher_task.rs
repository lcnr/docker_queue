use super::State;
use crate::domain::QueuedContainer;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

#[tracing::instrument(name = "Launcher task", skip(state, rx))]
pub(super) async fn start_launcher_task(
    state: Arc<State>,
    mut rx: mpsc::Receiver<QueuedContainer>,
) {
    while let Some(container) = rx.recv().await {
        info!("Received: {:?}", container);
        state.queued_containers.lock().unwrap().push(container);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn launcher_task_stores_queued_containers() {
        let state = Arc::new(State::new());
        let state2 = Arc::clone(&state);
        let (tx, rx) = mpsc::channel(1);
        tokio::spawn(async {
            start_launcher_task(state2, rx).await;
        });
        let container = QueuedContainer::new("docker run -d some_image").unwrap();
        tx.send(container.clone()).await.unwrap();
        sleep(Duration::from_millis(100)).await;
        assert!(state.queued_containers.lock().unwrap().contains(&container));
    }
}
