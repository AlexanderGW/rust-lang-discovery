pub fn run() {
	println!("Print from print::run()");

	println!("Number: {1}, {0}", 1, 2);

	println!("{name} likes to {verb} money", name ="Alex", verb = "get");

	println!("Binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

	println!("{:?}", (12, true, "string"));
}