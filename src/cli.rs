use std::env;

pub fn run() {
	let args: Vec<String> = env::args().collect();
	let command = args[1].clone(); // Or could &args[1]

	println!("Args:: {:?}", args);

	if command == "test" && !args[2].is_empty() {
		println!("We're testing: {}", args[2]);
	} else {
		println!("Option not available.");
	}
}