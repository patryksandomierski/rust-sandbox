#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn update_width(&mut self, width: u32) {
        self.width = width
    }
}

enum Message {
    Quit,
    Write(String),
}

impl Message {
    /// is it a doc?
    fn print_on_screen(&self) {
        println!("just printing... nothing exciting");
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 10,
        height: 15,
    };
    dbg!(&rect1);
    println!("rect area={}", area(&rect1));
    println!("rect area using impl={}", rect1.area());
    rect1.update_width(5);
    println!("rect area using impl={}", rect1.area());

    let rect2 = Rectangle::square(5);
    println!("square with side's length={} has area={}", rect2.height, rect2.area());

    let msg = Message::Write(String::from("hello"));
    msg.print_on_screen();
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
