pub fn run() {
	let mut count = 0;
	let max = 20;

	loop {
		count += 1;
		println!("Number: {}", count);

		if count == max {
			break;
		}
	}

	// While (fizzbuzz)
	// count = 0;
	// while count <= 100 {
	// 	if count % 15 == 0 {
	// 		println!("{}", "fizzbuzz");
	// 	} else if count % 3 == 0 {
	// 		println!("{}", "fizz");
	// 	} else if count % 5 == 0 {
	// 		println!("{}", "buzz");
	// 	} else {
	// 		println!("{}", count);
	// 	}

	// 	count += 1;
	// }

	// Range (fizzbuzz)
	for x in 0..100 {
		if x % 15 == 0 {
			println!("{}", "fizzbuzz");
		} else if x % 3 == 0 {
			println!("{}", "fizz");
		} else if x % 5 == 0 {
			println!("{}", "buzz");
		} else {
			println!("{}", x);
		}
	}
}