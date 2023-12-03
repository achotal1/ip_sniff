#![allow(dead_code)]

use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use tokio::net::{TcpStream};
use tokio::task;
use bpaf::Bpaf;
use std::sync::mpsc::{channel, Sender};

const IPFALLBACK:IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));


const MAX: u16 = 65535;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Store{
	#[bpaf(long, short, fallback(IPFALLBACK))]
	/// The address that you want to sniff. Must be valid ipv4
	pub address: IpAddr,
	#[bpaf(long("start"), short('s'), fallback(1u16), guard(start_port_guard, "Must be greater than 0"))]
	pub start_port: u16,
	#[bpaf(long("end"), short('e'), fallback(MAX), guard(end_port_guard, "Must be smaller than 65535"))]
	pub end_port: u16
}

fn start_port_guard(input: &u16) -> bool{
	*input > 0
}

fn end_port_guard(input: &u16) ->bool{
	*input <= MAX
}



async fn scan(tx: Sender<u16>, port: u16, ipaddr: IpAddr){
	
	match TcpStream::connect((ipaddr, port)).await{
		Ok(_) => {
						print!(".");
						io::stdout().flush().unwrap();
						tx.send(port).unwrap();
					 }
			Err(_) => {}
	}
			
}

#[tokio::main]
async fn main(){
	let ops: Store = store().run();
	let (tx, rx) = channel();

	for i in ops.start_port..ops.end_port{
		let t = tx.clone();
		task::spawn(async move {scan(t, i, ops.address).await});
	}
	let mut out = vec![];
	drop(tx);
	for i in rx{
		out.push(i);
	}
	out.sort();
	println!("{:?}", out);
}