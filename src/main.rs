#![allow(dead_code)]

use std::env::args;
use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::{process, io};
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX: u16 = 65535;

#[derive(Debug)]
struct Store{
	flag: String, 
	threads: u16,
	ip_addr: IpAddr
}

impl Store{
	fn new(args: &Vec<String>) -> Result<Store, &'static str>{
		if args.len() < 2{
			return Err("Need more arguments");
		}
		else if args.len() > 4{
			return Err("Too many Arguments");
		}
		let f = args[1].clone();
		if let Ok(ip) = IpAddr::from_str(&f){
			return Ok(Store{ flag: String::from(""), threads: 4, ip_addr: ip});
		}else{
			let flag = args[1].clone();
			if flag.contains("-h") || flag.contains("-help") {
				println!("-h or -help to print this message 
					or -j to add thread number");
				return Err("help called");
			}else if flag.contains("-j"){
				if args.len() < 4{
					return Err("Need more arguments");
				}
				
				let ip = match IpAddr::from_str(&args[3].clone()){
								 Ok(s) => s,
								 Err(_) => return Err("Invalid IP addr")
								};
				let thread_number = match args[2].parse::<u16>(){
									Ok(s) => s,
									Err(_) => return Err("Thread number cant be parsed")
								};

				return Ok(Store{flag: flag, threads: thread_number, ip_addr: ip});

			}else{
				return Err("Invalid syntax");
			}
		}
	}
}
fn scan(tx: Sender<u16>, start_port: u16, ipaddr: IpAddr, thread_no: u16 ){
	let mut port: u16 = start_port + 1;
	loop{
		match TcpStream::connect((ipaddr, port)){
			Ok(_) => {
						print!(".");
						io::stdout().flush().unwrap();
						tx.send(port).unwrap();
					 }
			Err(_) => {}
		}
		if MAX - port <= thread_no{
			break;
		}
		port += thread_no;
	}

}


fn main(){
	let args: Vec<_> = args().collect();
	let program_name = &args[0];
	let store: Store = Store::new(&args).unwrap_or_else(
							|_| {
								process::exit(0);
							}
						);
	let (tx, rx) = channel();
	let thread_no = store.threads;
	let ipaddr = store.ip_addr;
	for i in 0..thread_no{
		let t = tx.clone();
		thread::spawn(
					move ||{
						scan(t, i, ipaddr, thread_no);
					});
	}
	let mut out = vec![];
	drop(tx);
	for i in rx{
		out.push(i);
	}
	out.sort();
	println!("{:?}", out);
}