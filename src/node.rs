use serde::{Deserialize, Serialize};

use crate::{log::RaftLog, persist::Storage};

pub struct NodeConfig {
    pub id: u64,
    pub peers: Vec<u64>,
}

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

pub struct RaftNode<S> {
    config: NodeConfig,
    persistent: PersistentState,
    storage: S,
}

impl<S> RaftNode<S>
where
    S: Storage,
{
    pub async fn new(config: NodeConfig, storage: S) -> Self {
        let loaded_state = storage
            .load_state()
            .await
            .expect("Loading initial state failed");

        Self {
            config,
            storage,
            persistent: loaded_state.unwrap_or_default(),
        }
    }

    pub fn client_cmd(&mut self, cmd: &[u8]) {
        todo!()
    }
}
