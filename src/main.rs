use queue::Queue;
use std::str;

fn main() {
    let mut q = Queue::new();

    q.queue("Alessio dice: ".as_bytes()).unwrap();
    q.queue("ciao amici,".as_bytes()).unwrap();
    q.queue("scrivo dalla coda".as_bytes()).unwrap();

    while let Some(item) = q.dequeue() {
        println!("{}", str::from_utf8(&item).unwrap());
    }
}
