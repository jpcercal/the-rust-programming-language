enum Message {
    Write(String),
}

impl Message {
    fn get_message(&self) -> &str {
        match self {
            Message::Write(msg) => {
                msg
            }
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    println!("The message that must be written is \"{}\".", m.get_message());
}
