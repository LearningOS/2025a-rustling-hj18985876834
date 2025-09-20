// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("hello".into()));
    println!("{:?}", Message::Move{x:10,y:10});
    println!("{:?}", Message::ChangeColor(255,255,255));
}
