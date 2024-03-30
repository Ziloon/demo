fn main() {
    println!("{}", area((30, 50)));
}

fn area(dim:(u32, u32)) -> u32 {
    dim.0 * dim.1
}