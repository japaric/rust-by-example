use std::thread::Thread;

static NTHREADS: int = 10;

// This is the `main` thread
fn main() {
    for i in range(0, NTHREADS) {
        // Spin up another thread
        let _ = Thread::spawn(move || {
            println!("this is thread number {}", i)
        }).join();
    }
}
