use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let flag = args[1].clone();
    let threads = args[2].clone();
    let ip = args[3].clone();
}
