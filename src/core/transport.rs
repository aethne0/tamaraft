use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RaftMessage {
    AppendEntriesReq,
    AppendEntriesResp,
    RequestVoteReq,
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
pub trait Transport {
    fn send_raft(
        &mut self,
        destination_id: u64,
        msg: &RaftMessage,
    ) -> impl Future<Output = TransportResult<()>> + Send;

    fn recv_raft(
        &mut self,
        node_id: u64,
    ) -> impl Future<Output = TransportResult<Option<RaftMessage>>> + Send;
}
