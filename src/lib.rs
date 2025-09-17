pub mod core;

pub mod storage_adapters;
pub mod transport_adapters;

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::{
        core::{
            node::{NodeConfig, RaftNode},
            storage::Storage,
        },
        storage_adapters::in_memory::MemoryStorage,
        transport_adapters::in_process::InProcessTransport,
    };

    #[tokio::test]
    async fn test_all() {
        let cfg = NodeConfig {
            base_election_timeout: Duration::from_millis(300),
            heartbeat_timeout: Duration::from_millis(50),
            id: 1,
            peers: vec![],
        };

        let s = MemoryStorage::new();
        let t = InProcessTransport::new();

        let mut node = RaftNode::new(cfg, s, t).await;

        node.run().await;
    }
}
