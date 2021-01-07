struct Cli {
    tool: String,
    version: String
}

fn main() {

    let tool = std::env::args().nth(1).expect("no patern given");
    let version = std::env::args().nth(2).expect("no patern given");

    println!("Hello, world!");
}
