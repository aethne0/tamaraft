use crate::core::transport::Transport;

pub struct InProcessTransport {}

impl InProcessTransport {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Transport for InProcessTransport {
    async fn recv_rpc(&self, node_id: u64) -> Option<crate::core::transport::RaftRpcReq> {
        todo!()
    }

    async fn send_rpc(
        &self,
        destination_id: u64,
        rpc: crate::core::transport::RaftRpcReq,
    ) -> crate::core::transport::TransportResult<crate::core::transport::RaftRpcResp> {
        todo!()
    }
}
