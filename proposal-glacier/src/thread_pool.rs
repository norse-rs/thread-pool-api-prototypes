
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Mutex, Arc};
use super::*;

pub struct WorkQueue {
    tx: Sender<Box<dyn Task>>,
    rx: Arc<Mutex<Receiver<Box<dyn Task>>>>,
}

impl WorkQueue {
    pub fn new(num_threads: usize) -> Self {
        let (tx, rx): (Sender<Box<dyn Task>>, Receiver<Box<dyn Task>>) = channel();

        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..num_threads {
            let rx = Arc::clone(&rx);
            std::thread::spawn(move || {
                loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(mut job) => {
                            job.poll();
                        }
                        Err(..) => break,
                    }
                }
            });
        }

        WorkQueue {
            tx,
            rx,
        }
    }
}

impl Executor for WorkQueue {
    fn spawn(&mut self, f: impl Task) {
        self.tx.send(Box::new(f)).unwrap();
    }
}
