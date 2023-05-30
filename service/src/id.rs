use crate::config::Config;
use interfaces::IdGen;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

pub enum Request {
    Termination,
    NewKey {
        sharding_id: u64,
        num: usize,
        tx: oneshot::Sender<Vec<u64>>,
    }
}

pub struct IdGenerator {
    handle: Option<JoinHandle<()>>,
    started: AtomicBool,
    sender: Option<Sender<Request>>,
}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator {
            handle: None,
            started: AtomicBool::new(false),
            sender: None,
        }
    }

    pub fn sender(&self) -> Option<Sender<Request>> {
        self.sender.clone()
    }

    pub fn start<G: 'static + IdGen + Send + Sync>(&mut self, mut g: G, config: Config) {
        match self.started.compare_exchange(false, true, SeqCst, SeqCst) {
            Ok(false) => {}
            _ => return,
        };

        let (sender, mut rx) = tokio::sync::mpsc::channel(config.channel_size);
        self.sender = Some(sender);

        let handle = tokio::spawn(async move {
            loop {
                if let Some(k) = rx.recv().await {
                    match k {
                        Request::Termination => {
                            log::info!("stopped");
                            return;
                        }
                        Request::NewKey { sharding_id, num, tx } => {
                            let out = g.next_ids(sharding_id, num);
                            tx.send(out).unwrap();
                        }
                    }
                }
            }
        });
        self.handle = Some(handle);
    }
}
