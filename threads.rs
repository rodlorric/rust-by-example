use std::thread;

static NTHREADS: i32 = 6;
static NCOUNT: i32 = 10;

// This is the `main` thread.
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            for j in 0..NCOUNT {
                let t: f64 = (j as f64 * 3.1416) as f64 / ((j as f64) + 1.0);
                println!("Thread {}, t = {}", i, t);
            }
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finnish. Returns a result.
        let _ = child.join();
    }
}
