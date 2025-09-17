use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RaftRpcReq {
    AppendEntriesReq,
    RequestVoteReq,
}

#[derive(Serialize, Deserialize)]
pub enum RaftRpcResp {
    AppendEntriesResp,
    RequestVoteResp,
}

#[derive(Debug)]
pub enum TransportError {
    Unspecified,
    DeserializationError(String),
    SerializationError(String),
    SendError(String),
}
pub type TransportResult<T> = Result<T, TransportError>;

/// Abstract trait for message transport, to be implemented by user constructing a RaftNode
///
/// Node owns RaftMessages, Transport only borrows and creates them.
///
/// The transport implementation decides on the serialization method of RaftMessage as well.
pub trait Transport: Send + Sync {
    /// impl as `async`
    fn send_rpc(
        &self,
        destination_id: u64,
        rpc: RaftRpcReq,
    ) -> impl Future<Output = TransportResult<RaftRpcResp>> + Send;

    fn recv_rpc(&self, node_id: u64) -> impl Future<Output = Option<RaftRpcReq>> + Send;
}
