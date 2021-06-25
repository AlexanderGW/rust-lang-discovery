pub fn run() {
	foobar("Hello", "world!");

	// Bind return
	let sum = add(2, 4);
	println!("fn add() sum: {}", sum);

	// Closure
	let z: i32 = 10;
	let add_nums = |x: i32, y: i32| x + y + z;
	println!("Closure sum: {}", add_nums(4,6));
}

fn foobar(foo: &str, bar: &str) {
	println!("{} {}", foo, bar);
}

fn add(x: i32, y: i32) -> i32 {
	x + y
}