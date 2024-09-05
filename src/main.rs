use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;

struct ConcurrentQueue<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> ConcurrentQueue<T> {
    fn new(capacity: usize) -> Self {
        let (sender, receiver) = bounded(capacity);
        ConcurrentQueue { sender, receiver }
    }

    fn push(&self, value: T) -> Result<(), T> {
        self.sender.send(value).map_err(|e| e.0)  // Convert SendError<T> to T
    }

    fn get_receiver(&self) -> Receiver<T> {
        self.receiver.clone()
    }
}

fn main() {
    let queue = ConcurrentQueue::new(10);
    let receiver = queue.get_receiver();

    let producer = thread::spawn(move || {
        for i in 0..100 {
            match queue.push(i) {
                Ok(()) => println!("Produced: {}", i),
                Err(val) => println!("Failed to produce: {}", val),
            }
        }
    });

    let consumer = thread::spawn(move || {
        while let Ok(val) = receiver.recv() {
            println!("Received: {}", val);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
