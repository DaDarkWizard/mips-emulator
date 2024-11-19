mod computer;

fn main() {
    let mut com = computer::new(1, 1024);
    com.step();
}
