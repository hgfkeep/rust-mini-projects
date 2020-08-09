use std::env;
use std::io::{stdout, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};

const MAX_PORT: u16 = 65535;
struct Argument {
    threads: u16,
    address: IpAddr,
}

impl Argument {
    fn new(args: &[String]) -> Result<Argument, &'static str> {
        let help = "Usage: snifer [-h|--help| -t [thread_nums]| --threads [thread_nums]|] ip";
        if args.len() == 1 || (args.len() == 2 && args[1] == "-h" || args[1] == "--help") {
            return Err(help);
        } else {
            let address = match IpAddr::from_str(&args[args.len() - 1]) {
                Ok(add) => add,
                Err(_) => return Err("ip format error! user -h for more help"),
            };

            let threads = if args.len() == 4 {
                args[1].parse::<u16>().unwrap_or(1)
            } else {
                1
            };

            return Ok(Argument { threads, address });
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if MAX_PORT - port < num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument = match Argument::new(&args) {
        Ok(arg) => arg,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
    let (tx, rx) = channel();

    for i in 0..argument.threads {
        scan(tx.clone(), i, argument.address, argument.threads);
    }

    drop(tx);
    let mut res: Vec<u16> = Vec::new();

    for port in rx {
        res.push(port);
    }

    println!("");
    for port in res {
        println!("{}\tis open", port);
    }
}
