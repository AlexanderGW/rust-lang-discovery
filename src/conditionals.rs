pub fn run() {
	let age = 18;

	if age >= 18 {
		println!("{}", "All good");
	} else {
		println!("{}", "Nope");
	}

	// Shorthand
	let is_of_age = if age >= 18 { true } else { false };
	println!("Is of age? {}", is_of_age);
}