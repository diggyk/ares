use futures::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use tokio::prelude::*;
use tokio::runtime::Runtime;
use tokio::time::delay_for;
use tokio::time::Duration;

use super::broadcast::BroadcastMessage;

pub struct WebsocketServer {
    pub rx: Arc<Mutex<mpsc::Receiver<BroadcastMessage>>>,
}

impl WebsocketServer {
    async fn send_broadcast(&self, msg: BroadcastMessage) {
        // println!("Sending {:?}", msg);
    }

    async fn serve(&mut self) {
        let rx = &*self.rx.lock().unwrap();
        for received in rx {
            self.send_broadcast(received).await;
        }
    }

    pub fn run(&mut self) {
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let future = self.serve();
        rt.block_on(future);
    }
}
