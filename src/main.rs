use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    //take in string vector references as arguments, and return the argument struct in ok
    //portion (first) or static string in err portion (second). Used static to send back errors to
    //main function and have it handle the errors
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4})
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("Usage: -j to select how many threads you want \r\n -h or -help to show this help message")
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help"){
                return Err("too many arguments");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
}
