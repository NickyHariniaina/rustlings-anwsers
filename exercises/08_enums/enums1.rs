#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(String),
    Move(String),
    Echo(String),
    ChangeColor(String),
    Quit(String),
}

fn main() {
    println!("{:?}", Message::Resize(String::from("Hello")));
    println!("{:?}", Message::Move(String::from("Move")));
    println!("{:?}", Message::Echo(String::from("Echo")));
    println!("{:?}", Message::ChangeColor(String::from("CHange color")));
    println!("{:?}", Message::Quit(String::from("Quit")));
}
