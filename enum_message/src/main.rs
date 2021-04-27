enum Message {
    Quit,  // 枚举类型，没有关联任何数据
    Move {x: i32, y: i32}, // 包含匿名结构体的枚举成员
    Write(String),         // 包含 Sting 的
    ChangeColor(i32, i32, i32), // 包含三个 i32 的
}

impl Message {
    fn call(&self) {
        if let Message::Write(string) = self {
            println!("write: {}", string)
        }
        // match self {
        //     Message::Write(string) => println!("write: {}", string),
        //     _ => (),
        // }
    }
}


fn main() {
    let message = Message::Write(String::from("hello!"));

    message.call();
}
