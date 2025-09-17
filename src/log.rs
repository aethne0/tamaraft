use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct LogEntry {
    term: u64,
    index: u64,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RaftLog {
    inner: Vec<LogEntry>,
}

impl RaftLog {
    pub(crate) fn new() -> Self {
        Self { inner: Vec::new() }
    }
}
