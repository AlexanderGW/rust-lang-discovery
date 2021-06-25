pub fn run() {
	let mut hello = String::from("Hello");

	// String length
	println!("Length: {}", hello.len());

	println!("{}", hello);

	hello.push(',');

	hello.push_str(" world!");

	println!("{}", hello);

	// Byte size (capacity)
	println!("Capacity: {}", hello.capacity());

	// Empty?
	println!("Is emtpy: {}", hello.is_empty());

	// COntains
	println!("Contains 'world': {}", hello.contains("world"));

	// Replace
	println!("Replace: {}", hello.replace("world", "Earth"));

	// Loop string by whitespace
	for word in hello.split_whitespace() {
		println!("{}", word);
	}

	// Creatre string with capacity
	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');

	// Assertions
	assert_eq!(2, s.len());
	assert_eq!(10, s.capacity());

	println!("{}", s);
}