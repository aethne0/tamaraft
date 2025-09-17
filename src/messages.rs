/*
pub enum ClientReqError {
    Unspecified,
}
pub type ClientReqResult = Result<(), ClientReqError>;

pub enum ClientMessage<'a> {
    /// The raft implementation deals only with passing raw bytes to/from
    /// clients and the state machine. We are not going to do any type-ing
    /// to handle the inner commands. They will have to be deserialized by
    /// the user like normal anyway which will give some Result with
    /// associated type safety. In short, we do not care.
    ///
    /// By extension: To applying we simply pass the bytes to the state machine
    ClientReq(&'a [u8]),
    ClientResp(ClientReqResult),
}
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RaftMessage {
    AppendEntriesReq,
    AppendEntriesResp,
    RequestVoteReq,
    RequestVoteResp,
}
