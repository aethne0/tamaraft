use crate::core::storage::{PersistentState, Storage, StorageError, StorageResult};

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
        core::storage::{PersistentState, RaftLog, Storage},
        storage_adapters::in_memory::MemoryStorage,
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
