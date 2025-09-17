use crate::node::PersistentState;

#[derive(Debug)]
pub enum StorageError {
    Unspecified,
    DeserializationError(String),
    SerializationError(String),
}
pub type StorageResult<T> = Result<T, StorageError>;

/// Abstract trait for persistent storage
///
/// Node owns PersistentState
///
/// The storage implementation decides on the serialization method of PersistentState
/// as well.
pub trait Storage {
    fn load_state(&self) -> StorageResult<Option<PersistentState>> {
        todo!()
    }

    fn save_state(&mut self, _state: &PersistentState) -> StorageResult<()> {
        todo!()
    }
}

// For testing / example

/// Storage implementation that is only in-memory. It will not actually offer
/// persistence and is for testing purposes
///
/// MemoryStorage uses json for easy debugging purposes (human readable/editable)
pub struct MemoryStorage {
    inner: Option<String>,
}

impl MemoryStorage {
    #[must_use]
    pub fn new() -> Self {
        Self { inner: None }
    }
}

impl Storage for MemoryStorage {
    fn load_state(&self) -> StorageResult<Option<PersistentState>> {
        match &self.inner {
            Some(inner) => match &serde_json::from_str::<PersistentState>(&inner) {
                Ok(persistent_state) => StorageResult::Ok(Some(persistent_state.clone())),
                Err(e) => StorageResult::Err(StorageError::DeserializationError(e.to_string())),
            },
            None => StorageResult::Ok(None),
        }
    }

    fn save_state(&mut self, state: &PersistentState) -> StorageResult<()> {
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

#[test]
fn test_load_save() {
    let ps = PersistentState {
        current_term: 5,
        voted_for: Some(6),
        log: 1,
    };

    let mut st = MemoryStorage::new();

    st.save_state(&ps).unwrap();

    let ps_loaded = st.load_state().unwrap().unwrap();

    assert_eq!(ps_loaded.current_term, ps.current_term);
    assert_eq!(ps_loaded.voted_for, ps.voted_for);
    assert_eq!(ps_loaded.log, ps.log);
}
