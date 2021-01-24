use queue::Queue;
use std::collections::HashMap;
use std::str;
use std::vec::Vec;

struct Broker<'a> {
    channels: HashMap<&'a str, usize>,
    indexes: Vec<queue::Queue<&'a str>>,
}

impl<'a> Broker<'a> {
    
    pub fn publish(&mut self, channel: &'a str, payload: &'a str) {
        self.indexes[self.channels[channel]].queue(payload);
    }

    pub fn create_channel(&mut self, channel: &'a str) {
        self.indexes.push(Queue::new());
        self.channels.insert(channel, self.indexes.len()-1);
    }

}

fn main() {

    let mut broker = Broker{
        channels: HashMap::new(),
        indexes: Vec::new()
    };

    broker.create_channel("hello");   
    broker.publish("hello", "Alessio ");  
    broker.publish("hello", "Proietti ");  
    broker.publish("hello", "mangia ");  
    broker.publish("hello", "gli ");  
    broker.publish("hello", "spaghetti");  


    while let Some(item) = broker.indexes[broker.channels["hello"]].dequeue() {
        println!("{}", item);
    }
}
