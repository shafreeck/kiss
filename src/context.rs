use std::sync::Arc;
use std::vec::Vec;

use tokio::sync::oneshot;

use crate::command::{Command, Desc};
use crate::resp::Kind;
use crate::storage::Storage;
use crate::storage::db::Db;

pub struct ServerContext {
    pub auth: String,
    pub clients: Vec<ClientContext>,
}

pub struct ClientContext {
    pub authenticated: bool,
}

// this is the context to run a command
// the context should include these states
//  * client:  all states maintained for a client, Ex. Is this client
// authenticated, what is the current db, is it in multi mode
//  * server: all states maintained for the server, Ex. configration
//  * command: command name, args and its description
// the description contains two things: the function to execute and the
// constrains of the command
//
// the conext has the same lifetime with a command
pub struct Context {
    pub cli: Arc<ClientContext>,
    pub srv: Arc<ServerContext>,
    pub cmd: Arc<Command>,
    pub desc: Arc<&'static Desc>,
    pub sink: oneshot::Sender<Kind>,
}
