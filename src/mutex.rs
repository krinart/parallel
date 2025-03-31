use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::Thread;

// A simple mutex implementation
pub struct SimpleMutex<T> {
    // The lock state: false = unlocked, true = locked
    locked: AtomicBool,
    
    // The data protected by the mutex
    // UnsafeCell allows for internal mutability
    data: UnsafeCell<T>,
}

// A guard that provides access to the mutex's data and releases the lock when dropped
pub struct SimpleMutexGuard<'a, T> {
    mutex: &'a SimpleMutex<T>,
}

// Implement Send for SimpleMutex if T implements Send
// This means the mutex can be sent between threads
unsafe impl<T: Send> Send for SimpleMutex<T> {}

// Implement Sync for SimpleMutex if T implements Send
// This means the mutex can be shared between threads
unsafe impl<T: Send> Sync for SimpleMutex<T> {}

impl<T> SimpleMutex<T> {
    // Create a new mutex containing the given data
    pub fn new(data: T) -> Self {
        SimpleMutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    
    // Attempt to lock the mutex
    pub fn lock(&self) -> SimpleMutexGuard<T> {
        // Try to acquire the lock using compare_exchange
        // This is a spin lock implementation for simplicity
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // If we couldn't acquire the lock, yield to the scheduler
            // to avoid wasting CPU cycles
            thread::yield_now();
        }
        
        // Return a guard that grants access to the data
        SimpleMutexGuard { mutex: self }
    }
}

impl<'a, T> Deref for SimpleMutexGuard<'a, T> {
    type Target = T;
    
    // Allow dereferencing the guard to access the inner data
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for SimpleMutexGuard<'a, T> {
    // Allow mutable dereferencing of the guard
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for SimpleMutexGuard<'a, T> {
    // Release the lock when the guard is dropped
    fn drop(&mut self) {
        // Set the locked flag back to false
        self.mutex.locked.store(false, Ordering::Release);
    }
}

// Example usage
fn main() {
    // Create a mutex containing a counter
    let counter = SimpleMutex::new(0);
    
    // Create threads that increment the counter
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_ref = &counter;
        let handle = thread::spawn(move || {
            // Lock the mutex to get exclusive access
            let mut guard = counter_ref.lock();
            
            // Modify the data
            *guard += 1;
            
            // The lock is automatically released when guard goes out of scope
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print the final value
    let final_count = *counter.lock();
    println!("Final count: {}", final_count);
}