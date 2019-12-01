use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let file = File::open("input.txt");
	let file = match file {
		Ok(file) => file,
		Err(error) => {
			panic!("Couldn't open file got error: {:?}", error);
		}
	};
	let reader = BufReader::new(file);
	let mut module_weights: Vec<i32> = Vec::new();

	for line in reader.lines() {
		let line = match line {
			Ok(line) => line,
			Err(error) => panic!("Couldn't read line got error: {:?}", error),
		};

		let module_weight = line.parse::<i32>();
		let module_weight = match module_weight {
			Ok(m) => m,
			Err(error) => panic!("Couldn't parse line into number got error: {:?}", error),
		};

		module_weights.push(module_weight);
	}
	let per_module_fuel_requirement: i32 = module_weights
		.into_iter()
		.map(calculate_fuel_requirement)
		.fold(0, |acc, x| acc + x);
	println!("{:?}", per_module_fuel_requirement);
}

fn calculate_fuel_requirement(module_weight: i32) -> i32 {
	(module_weight / 3) - 2
}
