#[derive(Debug)]
struct Cubox {
    width: u32,
    length: u32,
}

impl Cubox {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}
fn main() {
    let rect = Cubox {
        width: 30,
        length: 50,
    };
    println!("{}", rect.area());
    println!("{:#?}", rect);
}

