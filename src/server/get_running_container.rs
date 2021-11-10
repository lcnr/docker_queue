use super::State;
use crate::domain::RunningContainerId;
use axum::{extract::Extension, Json};
use std::sync::Arc;

#[tracing::instrument(name = "Get running container", skip(state))]
pub(super) async fn get_running_container(
    Extension(state): Extension<Arc<State>>,
) -> Json<Option<RunningContainerId>> {
    let container_id = state.get_running_container();
    Json(container_id)
}

impl State {
    pub(super) fn get_running_container(&self) -> Option<RunningContainerId> {
        self.running_container.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_running_container_works() {
        let state = State::new();
        assert!(state.get_running_container().is_none());
        let id = RunningContainerId::new("123456");
        *state.running_container.lock().unwrap() = Some(id);
        assert!(state.get_running_container().is_some());
    }
}
