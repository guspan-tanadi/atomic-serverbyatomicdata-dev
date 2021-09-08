//! The Commit Monitor checks for new commits and notifies listeners.
//! It is mostly used by WebSockets to notify front-end clients of changes Resources.

// TODO: define messages between CommitMonitor and WebSocketConnection
use crate::{
    actor_messages::{CommitMessage, Subscribe},
    handlers::web_sockets::WebSocketConnection,
};
use actix::{
    prelude::{Actor, Context, Handler},
    Addr,
};
use std::collections::{HashMap, HashSet};

// We might need this instead of Addr, but I don't understand it
// type Socket = Recipient<WsMessage>;

/// The Commit Monitor is an Actor that checks for new commits and notifies listeners.
pub struct CommitMonitor {
    /// Maintains a list of all the resources that are being subscribed to, and maps these to websocket connections.
    subscriptions: HashMap<String, HashSet<Addr<WebSocketConnection>>>,
}

impl CommitMonitor {}

// Since his Actor only starts once, there is no need to handle its lifecycle
impl Actor for CommitMonitor {
    type Context = Context<Self>;
}

impl Default for CommitMonitor {
    fn default() -> CommitMonitor {
        CommitMonitor {
            subscriptions: HashMap::new(),
        }
    }
}

impl Handler<Subscribe> for CommitMonitor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Context<Self>) {
        let mut set = if let Some(set) = self.subscriptions.get(&msg.subject) {
            set.clone()
        } else {
            HashSet::new()
        };
        set.insert(msg.addr);
        log::info!("handle subscribe {} ", msg.subject);
        self.subscriptions.insert(msg.subject, set);
    }
}

impl Handler<CommitMessage> for CommitMonitor {
    type Result = ();

    fn handle(&mut self, msg: CommitMessage, _: &mut Context<Self>) {
        log::info!(
            "handle commit for {} with id {}. Current connections: {}",
            msg.subject,
            msg.resource.get_subject(),
            self.subscriptions.len()
        );
        if let Some(set) = self.subscriptions.get(&msg.subject) {
            log::info!("Updating commit {} for {} sockets", msg.subject, set.len());
            for connection in set {
                log::info!("Sending commit for connection");
                connection.do_send(msg.clone());
            }
        } else {
            log::info!("No subscribers for {}", msg.subject);
        }
    }
}
