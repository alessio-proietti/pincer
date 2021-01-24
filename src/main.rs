use queue::Queue;
use std::collections::HashMap;
use std::str;
use std::vec::Vec;

fn main() {
    //let mut q = Queue::new();
    let mut vec = Vec::new();

    let mut map = HashMap::new();
    vec.push(Queue::new());
    map.insert("hello", 0);

    vec[map["hello"]].queue("Alessio dice: ".as_bytes()).unwrap();
    vec[map["hello"]].queue("ciao amici,".as_bytes()).unwrap();
    vec[map["hello"]].queue("scrivo dalla coda".as_bytes()).unwrap();

    while let Some(item) = vec[map["hello"]].dequeue() {
        println!("{}", str::from_utf8(&item).unwrap());
    }
}
