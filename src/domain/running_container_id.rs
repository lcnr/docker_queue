pub struct RunningContainerId(String);

impl RunningContainerId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl AsRef<str> for RunningContainerId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
