// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
// use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::{Receiver, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    // sender: todo!(),
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    // pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, todo!()> {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<TicketDraft>> {
        // todo!()

        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        let _ = self.sender.try_send(Command::Insert {
            draft,
            response_channel: response_sender,
        });

        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<TicketId>> {
        // todo!()

        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);

        let _ = self.sender.try_send(Command::Get {
            id,
            response_channel: response_sender,
        });

        Ok(response_receiver.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    // todo!();
    //
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    //
    std::thread::spawn(move || server(receiver));
    // todo!()

    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        // response_channel: todo!(),
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        // response_channel: todo!(),
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                // todo!()
                // let _ = response_channel.try_send(id);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                // todo!()
                // let _ = response_channel.try_send(ticket.cloned());
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
