use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningContainerId(String);

impl RunningContainerId {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into().trim().into();
        Self(id)
    }
}

impl AsRef<str> for RunningContainerId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_container_id_should_not_have_whitespaces() {
        let id = RunningContainerId::new("12345 ");
        assert_eq!("12345", id.as_ref());
        let id = RunningContainerId::new("12345\n");
        assert_eq!("12345", id.as_ref());
    }
}
