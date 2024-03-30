#[derive(Debug)]
struct Cubox {
    width: u32,
    length: u32,
}

impl Cubox {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, other: &Cubox) -> bool {
        self.width >= other.width && self.length >= other.length
    }
    fn square(size: u32) -> Cubox {
        Cubox {
            width: size,
            length: size,
        }
    }
}
fn main() {
    let rect1 = Cubox {
        width: 30,
        length: 50,
    };
    println!("{}", rect1.area());
    println!("{:#?}", rect1);
    let rect2 = Cubox {
        width: 30,
        length: 50,
    };
    println!("{}", rect1.can_hold(&rect2));
    let sq = Cubox::square(50);
    println!("{}", sq.can_hold(&rect2));
    println!("{}", rect2.can_hold(&sq));
}

