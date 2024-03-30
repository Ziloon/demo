#[derive(Debug)]
struct Cubox {
    width: u32,
    length: u32,
}

fn main() {
    let rect = Cubox {
        width: 30,
        length: 50,
    };
    println!("{}", area(&rect));
    println!("{:#?}", rect);
}

fn area(rect: &Cubox) -> u32 {
    rect.width * rect.length
}