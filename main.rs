use std::io::{Write, Read};
use std::fs::File;
use std::path::Path;
use clap::App;
use clap::Arg;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct De {
	services: Vec<Service> 
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Service {
	name: String,
	num: i32
}

fn main() {
	let matches = App::new("usagestats")
		.version("0.1.0")
		.about("Easy service usage statictics collector")
		.author("Aydar N.")
		.arg(Arg::with_name("SERVICE")
			.help("Service name")
			.required(true)
			.index(1))
		.arg(Arg::with_name("increment")
			.short("i")
			.long("increment")
			.help("Add one new usage report to a service"))
		.arg(Arg::with_name("display")
			.short("d")
			.long("display")
			.help("Display usage stats for a service"))
		.arg(Arg::with_name("reset")
			.short("r")
			.long("reset")
			.help("Sets special service usages number to nil"))
		.get_matches();
	let path = Path::new("./usage.json");
	let ser = matches.value_of("SERVICE").unwrap().to_string();

	if !path.exists(){
		println!("File not found, creating new...");
		let mut file = File::create(path).unwrap();
		let dummy = De{services: vec![]};
		let serialized = serde_json::to_string(&dummy).unwrap();
		write!(&mut file, "{}", serialized).unwrap();
	}
	let mut file = File::open(path).expect("Fatal fs error");
	let mut data = "".to_string();
	file.read_to_string(&mut data).expect("Fatal fs error");
	let mut serde:De = serde_json::from_str(&data).unwrap();

	let mut servlist = vec![];
	for service in serde.services.clone() {
		servlist.push(service.name);
	}

	if matches.is_present("increment"){
		if servlist.contains(&&ser) {
			let numb = serde.services[servlist.iter().position(|r| r == &ser).unwrap()].num;
			serde.services[servlist.iter().position(|r| r == &ser).unwrap()] = Service{name: ser.clone(), num: numb + 1};
		}
		else {
			serde.services.push(Service{name: ser, num: 1})
		}
		let mut file = File::create(path).unwrap();
		let serialized = serde_json::to_string(&serde).unwrap();
		write!(&mut file, "{}", serialized).unwrap();
	}
	else if matches.is_present("display"){
		if servlist.contains(&&ser) {
			print!("{:?}", serde.services[servlist.iter().position(|r| r == &ser).unwrap()].num);
		}
		else {
			panic!("No such service");
		}
	}
	else if matches.is_present("reset"){
		if servlist.contains(&&ser) {
			serde.services[servlist.iter().position(|r| r == &ser).unwrap()] = Service{name: ser.clone(), num: 0};
		}
		else {
			panic!("No such service");
		}
		let mut file = File::create(path).unwrap();
		let serialized = serde_json::to_string(&serde).unwrap();
		write!(&mut file, "{}", serialized).unwrap();
	}
}
