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
	let module_weights = vec!<i32>();
	for line in reader.lines() {
		let line = match line {
			Ok(line) => line,
			Err(error) => panic!("Couldn't read line got error: {:?}", error),
		};

		println!("{}", line)
	}
}
