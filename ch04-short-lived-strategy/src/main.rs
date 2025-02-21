#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to_sat: u64,
    content: String,
}

struct GroundStation {}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for (index, message) in self.messages.iter().enumerate() {
            if message.to_sat == recipient.id {
                let msg = self.messages.remove(index);
                return Some(msg);
            }
        }
        None
    }
}

fn main() {
    println!("Hello, world!");
}
