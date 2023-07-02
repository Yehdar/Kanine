use std::env;

struct Arguments {
    flag: String,
    ip: Ip,
    threads: u16,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
}
