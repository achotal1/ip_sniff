#![allow(dead_code)]

use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::{TcpStream};
use tokio::task;
use bpaf::Bpaf;
use std::sync::mpsc::{channel, Sender};

const IPFALLBACK:IpvAddr = IpvAddr::V4(Ipv4Addr::new(127, 0, 0, 1));


const MAX: u16 = 65535;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Store{
	#[bpaf(long, short, fallback(IPFALLBACK))]
	/// The address that you want to sniff. Must be valid ipv4
	pub address: IpAddr,
	#[bpaf(long("start"), short("s"), fallback(1u16), guard(start_port_guard, "Must be greater than 0"))]
	pub start_port: u16,
	#[bpaf(long("end"), short("e"), fallback(MAX), guard(end_port_guard, "Must be smaller than 65535"))]
	pub end_port: u16
}

fn start_port_guard(input: &u16) -> bool{
	*input > 0
}

fn end_port_guard(input: &u16) ->bool{
	*input <= MAX
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