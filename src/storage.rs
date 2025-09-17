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

// For testing / example

/// Storage implementation that is only in-memory. It will not actually offer
/// persistence and is for testing purposes
///
/// MemoryStorage uses json for easy debugging purposes (human readable/editable)
pub struct MemoryStorage {
    inner: Option<String>,
}

impl MemoryStorage {}

impl Storage for MemoryStorage {
    fn new() -> Self {
        Self { inner: None }
    }

    /// Loads persistent state from storage
    async fn load_state(&self) -> StorageResult<Option<PersistentState>> {
        match &self.inner {
            Some(inner) => match &serde_json::from_str::<PersistentState>(&inner) {
                Ok(persistent_state) => StorageResult::Ok(Some(persistent_state.clone())),
                Err(e) => StorageResult::Err(StorageError::DeserializationError(e.to_string())),
            },
            None => StorageResult::Ok(None),
        }
    }

    /// Saves persistent state from storage
    async fn save_state(&mut self, state: &PersistentState) -> StorageResult<()> {
        let ser = serde_json::to_string(state);

        match ser {
            Ok(string) => {
                self.inner = Some(string);
                return StorageResult::Ok(());
            }
            Err(e) => StorageResult::Err(StorageError::DeserializationError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        log::RaftLog,
        storage::{MemoryStorage, PersistentState, Storage},
    };

    #[tokio::test]
    async fn test_load_save() {
        let ps = PersistentState {
            current_term: 5,
            voted_for: Some(6),
            log: RaftLog::new(),
        };

        let mut st = MemoryStorage::new();

        st.save_state(&ps).await.unwrap();

        let ps_loaded = st.load_state().await.unwrap().unwrap();

        assert_eq!(ps_loaded.current_term, ps.current_term);
        assert_eq!(ps_loaded.voted_for, ps.voted_for);
        assert_eq!(ps_loaded.log, ps.log);
    }
}
