use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;

pub struct Sender<T> {
    // Shared state between sender and receiver
    inner: Arc<Inner<T>>,
}

pub struct Receiver<T> {
    // Same shared state as sender
    inner: Arc<Inner<T>>,
}

// The shared state between sender and receiver
struct Inner<T> {
    // Queue of messages
    queue: Mutex<VecDeque<T>>,
    // Condition variable to notify the receiver when data is available
    available: Condvar,
    // Flag to indicate if the sender has been dropped
    is_closed: Mutex<bool>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // Create the shared state
    let inner = Arc::new(Inner {
        queue: Mutex::new(VecDeque::new()),
        available: Condvar::new(),
        is_closed: Mutex::new(false),
    });

    // Create sender and receiver with shared state
    let sender = Sender {
        inner: Arc::clone(&inner),
    };

    let receiver = Receiver {
        inner: Arc::clone(&inner),
    };

    (sender, receiver)
}

impl<T> Sender<T> {
    pub fn send(&self, item: T) -> Result<(), SendError<T>> {
        // Check if the channel is closed (receiver dropped)
        let is_closed = *self.inner.is_closed.lock().unwrap();
        if is_closed {
            return Err(SendError(item));
        }

        // Lock the queue and push the item
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(item);
        
        // Notify the receiver that data is available
        self.inner.available.notify_one();
        
        Ok(())
    }
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T, RecvError> {
        // Lock the queue
        let mut queue = self.inner.queue.lock().unwrap();
        
        // Wait until there's an item or the channel is closed
        loop {
            // Check if there's an item in the queue
            if let Some(item) = queue.pop_front() {
                return Ok(item);
            }
            
            // Check if the channel is closed (sender dropped)
            let is_closed = *self.inner.is_closed.lock().unwrap();
            if is_closed {
                return Err(RecvError);
            }
            
            // Wait for notification that data is available
            queue = self.inner.available.wait(queue).unwrap();
        }
    }
    
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        // Lock the queue
        let mut queue = self.inner.queue.lock().unwrap();
        
        // Try to get an item without waiting
        if let Some(item) = queue.pop_front() {
            return Ok(item);
        }
        
        // Check if the channel is closed
        let is_closed = *self.inner.is_closed.lock().unwrap();
        if is_closed {
            return Err(TryRecvError::Disconnected);
        }
        
        // No data available
        Err(TryRecvError::Empty)
    }
}

// Drop implementations to handle disconnection

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Mark the channel as closed when the sender is dropped
        let mut is_closed = self.inner.is_closed.lock().unwrap();
        *is_closed = true;
        
        // Notify the receiver in case it's waiting
        self.inner.available.notify_all();
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // Mark the channel as closed when the receiver is dropped
        let mut is_closed = self.inner.is_closed.lock().unwrap();
        *is_closed = true;
    }
}

// Error types

#[derive(Debug)]
pub struct SendError<T>(pub T);

#[derive(Debug)]
pub struct RecvError;

#[derive(Debug)]
pub enum TryRecvError {
    Empty,
    Disconnected,
}

// Example usage
fn example() {
    let (tx, rx) = channel::<String>();
    
    std::thread::spawn(move || {
        tx.send(String::from("Hello")).unwrap();
    });
    
    let msg = rx.recv().unwrap();
    assert_eq!(msg, "Hello");
}