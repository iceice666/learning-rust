#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let scale = 2;
    let rec1 = Rectangle {
        width: 30,
        // width: gbg!(scale * 30),
        height: 50,
    };

    // println!("rec1 is {:?}", rec1);
    dbg!(&rec1);

    println!("area of rectangle is {}", rec1.area());
}
