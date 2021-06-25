pub fn run() {

	// Immutable by default
	let name = "Alex";

	// Mutable
	let mut number = 7;
	println!("{} likes the number {}", name, number);
	number = 9;
	println!("{} likes the number {}", name, number);

	// Constant must be type casted
	const ID: i64 = 000000000000004;
	println!("ID: {}", ID);

	// Multiple values
	let (a1, a2) = ("b1", "b2");
	println!("{} and {}", a1, a2);
}