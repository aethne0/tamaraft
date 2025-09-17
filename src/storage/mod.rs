pub mod in_memory;

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

/// Fields that must be persisted during RPC methods and used in the case of node-recovery
#[derive(Deserialize, Serialize, Clone)]
pub struct PersistentState {
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub log: RaftLog,
}
impl Default for PersistentState {
    fn default() -> Self {
        Self {
            current_term: 1,
            voted_for: None,
            log: RaftLog::new(),
        }
    }
}

#[derive(Debug)]
pub enum StorageError {
    Unspecified,
    DeserializationError(String),
    SerializationError(String),
}
pub type StorageResult<T> = Result<T, StorageError>;

/// Abstract trait for persistent storage, to be implemented by user constructing a RaftNode
///
/// Node owns PersistentState
///
/// The storage implementation decides on the serialization method of PersistentState
/// as well.
pub trait Storage {
    /// New is not async - first time initialiation that may have need for async can be done
    /// in the first write/read if need be.
    fn new() -> Self;

    // ?    Desugured from async according to https://users.rust-lang.org/t/async-in-public-trait/108400/2
    // ?    I am putting this here for later because I don't fully understand the +/- Send implications
    // ?    explained in the lint/answer.
    fn load_state(&self) -> impl Future<Output = StorageResult<Option<PersistentState>>> + Send;

    fn save_state(
        &mut self,
        _state: &PersistentState,
    ) -> impl Future<Output = StorageResult<()>> + Send;
}
