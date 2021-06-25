pub fn run() {
	let person: (&str, &str, i8) = ("Alex", "UK", 7);

	println!("{} is from {}, fav number is {}", person.0, person.1, person.2);
}