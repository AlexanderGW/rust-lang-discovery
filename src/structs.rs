
// Traditional
struct Colour {
	red: u8,
	green: u8,
	blue: u8,
}

struct Person {
	first_name: String,
	last_name: String
}

// Tuple
struct Colour2(u8, u8, u8);

impl Person {

	// Construct
	fn new(first: &str, last: &str) -> Person {
		Person {
			first_name: first.to_string(),
			last_name: last.to_string(), // Superfulous commas are acceptable
		}
	}

	// Working with a struct instance
	fn full_name(&self) -> String {
		format!("{} {}", self.first_name, self.last_name) // NOTE: Returns don't include semicolon - curious as to why this requirement
	}

	// Mutate a struct value
	fn set_last_name(&mut self, last: &str) {
		self.last_name = last.to_string();
	}

	// To tuple
	fn to_tuple(self) -> (String, String) { // NOTE: 'self' not passed as reference here - curious as to why this requirement
		(self.first_name, self.last_name)
	}
}

pub fn run() {
	let mut c = Colour {
		red: 255,
		green: 0,
		blue: 0
	};

	c.red = 128;

	println!("Colour: {} {} {}", c.red, c.green, c.blue);

	let c2 = Colour2(255, 0, 0);

	println!("Colour2: {} {} {}", c2.0, c2.1, c2.2);

	let mut p = Person::new("Alex", "White");
	println!("Person: {} {}", p.first_name, p.last_name);

	println!("Full name: {}", p.full_name());

	p.set_last_name("Pink");

	println!("Full name (mutated): {}", p.full_name());

	println!("Person to tuple: {:?}", p.to_tuple());
}