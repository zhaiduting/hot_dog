use std::cell::RefCell;

struct Messenger {
    sent_messages: Vec<String>,
}

impl Messenger {
    fn new() -> Self {
        Messenger {
            sent_messages: Vec::new(),
        }
    }

    // ❌ 这里用了 &self，但试图修改字段
    fn send(&mut self, message: &str) {
        self.sent_messages.push(String::from(message));
    }
}
fn fish() -> Option<usize> {
    let x: RefCell<Vec<String>> = RefCell::new(Vec::new());
    None
}
fn main() {
    let mut messenger = Messenger::new();
    messenger.send("Hello, Rust!");
    println!("Messages: {:?}", messenger.sent_messages);
}
