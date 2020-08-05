#[allow(unused_imports)]
use futures::prelude::*;
#[allow(unused_imports)]
use tokio::prelude::*;

use futures::channel::mpsc::UnboundedSender;
use serde_json;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use tokio::runtime::Runtime;
use tokio::sync::RwLock;
use tokio::time::{delay_for, Duration};
use warp::ws::{Message, WebSocket};
use warp::{Error, Filter};

use super::broadcast::BroadcastMessage;

// global client id counter
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

/// Active clients
type Clients = Arc<RwLock<HashMap<usize, UnboundedSender<Result<Message, Error>>>>>;

/// MPSC receiver from the server
type StandardReceiver = Arc<Mutex<std::sync::mpsc::Receiver<BroadcastMessage>>>;

/// MPSC transmitter to the server
type StandardSender = Arc<Mutex<std::sync::mpsc::Sender<usize>>>;

pub struct WebsocketServer {
    pub server_rx: StandardReceiver,
    pub server_tx: StandardSender,
    pub clients: Clients,
}

impl WebsocketServer {
    pub fn new(server_rx: StandardReceiver, server_tx: StandardSender) -> Self {
        WebsocketServer {
            server_rx,
            server_tx,
            clients: Clients::default(),
        }
    }

    async fn send_broadcast(clients: Clients, msg: BroadcastMessage) {
        let mut dead_clients: Vec<usize> = Vec::new();

        let for_client: Option<usize> = match msg {
            BroadcastMessage::InitializerData {
                id,
                cells: _,
                robots: _,
                valuables: _,
            } => Some(id),
            _ => None,
        };

        if let Ok(msg_json) = serde_json::to_string(&msg) {
            for (id, mut tx) in clients.read().await.iter() {
                if for_client.is_some() && *id != for_client.unwrap() {
                    continue;
                }

                let _msg = msg_json.clone();
                if let Err(e) = tx.send(Ok(Message::text(_msg))).await {
                    println!("Error sending to {}: {}", id, e);
                    dead_clients.push(*id);
                }
            }
        } else {
            println!("ERROR: failed to serialize {:#?}", msg);
        }
        for id in &dead_clients {
            let _ = Self::listener_disconnected(*id, clients.clone()).await;
        }
    }

    async fn listener_connected(ws: WebSocket, clients: Clients, server_tx: StandardSender) {
        let client_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

        println!("Listener {} connected", client_id);

        let (client_ws_tx, _) = ws.split();
        let (tx, rx) = futures::channel::mpsc::unbounded::<Result<Message, Error>>();

        tokio::task::spawn(rx.forward(client_ws_tx).map(|result| {
            if let Err(e) = result {
                println!("Websocket send error: {}", e);
            }
        }));

        clients.write().await.insert(client_id, tx);
        let _ = server_tx.lock().unwrap().send(client_id);
    }

    async fn listener_disconnected(id: usize, client_list: Clients) {
        println!("Listener {} disconnected", id);
        client_list.write().await.remove(&id);
    }

    /// async loop that listens for incoming broadcast messages from the
    /// server and rebroadcasts to all listeners
    async fn rebroadcast_loop(rx: StandardReceiver, clients: Clients) {
        loop {
            let msg_try = rx.lock().unwrap().try_recv();
            if let Ok(msg) = msg_try {
                Self::send_broadcast(clients.clone(), msg).await;
            } else {
                delay_for(Duration::from_secs(1)).await;
            }
        }
    }

    async fn serve(&self) {
        let client_list = self.clients.clone();
        let _server_tx = self.server_tx.clone();
        let clients = warp::any().map(move || client_list.clone());
        let server_tx = warp::any().map(move || _server_tx.clone());

        let listen = warp::path("listen")
            .and(warp::ws())
            .and(clients)
            .and(server_tx)
            .map(|ws: warp::ws::Ws, clients, server_tx| {
                ws.on_upgrade(move |socket| {
                    WebsocketServer::listener_connected(socket, clients, server_tx)
                })
            });

        tokio::task::spawn(Self::rebroadcast_loop(
            self.server_rx.clone(),
            self.clients.clone(),
        ));

        warp::serve(listen).run(([127, 0, 0, 1], 3820)).await;
    }

    pub fn run(&mut self) {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(self.serve());
    }
}
