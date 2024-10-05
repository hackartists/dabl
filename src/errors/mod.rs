use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DablException {
    UnknownException(String),
}

impl std::fmt::Display for DablException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for DablException {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(DablException::UnknownException(s.to_string()))
    }
}

impl std::error::Error for DablException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl DablException {
    pub fn to_string(&self) -> String {
        format!("BaseError: {:?}", self)
    }
}

unsafe impl Send for DablException {}
unsafe impl Sync for DablException {}
