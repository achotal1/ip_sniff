#![allow(dead_code)]
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;

#[derive(Debug)]
struct Arguments{	
	flag: String,
	ipaddress: IpAddr,
	threads: u16
}

impl Arguments{
	fn new(args: &Vec<String> ) -> Result<Arguments, &'static str>{
		if args.len() < 2{
			return Err("Not enough arguments");
		}else if args.len() > 4{
			return Err("Too many arguments");
		}
		let f = args[1].clone();
		if let Ok(ipaddr) = IpAddr::from_str(&f){
			return Ok(Arguments{flag: String::from(""), ipaddress: ipaddr, threads: 4});
		}else{
			let flag = args[1].clone();
			if flag == "-h" || flag == "-help"{
				if args.len() > 2 {
					return Err("Too many arguments");
				}else{
					println!("Usage: -j for adding number of threads
							  \n -h or -help for this message");
					return Err("Help");
				}
			}
			else if flag == "-j"{
				let ipaddr = match IpAddr::from_str(&args[3].clone()){
									Ok(ans) => ans,
									Err(_) => return Err("IP Address is not valid")
							};
				let thread = match args[2].parse::<u16>(){
									Ok(ans) => ans,
									Err(_)  => return Err("failed to parse thread number")
							  };
				return Ok(Arguments{flag: f,ipaddress: ipaddr,threads: thread});
			}
			else{
				return Err("Invalid Syntax");
			}
		
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();
	let m: Arguments = Arguments::new(&args).unwrap_or_else(
						|err| {
							if err.contains("help"){
								process::exit(0);
							}else{
								eprintln!("{0} problem parsing arguments: {1}", program, err);
								process::exit(0);
							}

						});
	println!("{:?}", m);
}
