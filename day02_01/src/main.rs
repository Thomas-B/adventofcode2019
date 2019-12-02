use std::fs;

fn main() {
	let input = fs::read_to_string("input.txt");
	let input = match input {
		Ok(i) => i,
		Err(error) => panic!("Couldn't read file got error: {:?}", error),
	};

	let mut op_codes: Vec<i32> = input
		.trim()
		.split(",")
		.into_iter()
		.map(|o| {
			let n = o.parse::<i32>();
			let n = match n {
				Ok(n) => n,
				Err(error) => panic!("Couldn't read file got error: {:?} {:?}", error, o),
			};
			n
		})
		.collect();

	process_gravity_assist_program(&mut op_codes);
	println!("{:?}", op_codes);
}

fn process_gravity_assist_program(program: &mut Vec<i32>) {
	let mut cont = true;
	let mut i = 0;

	while cont {
		let op_code = program[i];
		if op_code == 1 {
			let first_index = i + 1;
			let second_index = i + 2;
			let result_index = program[i + 3] as usize;

			program[result_index] = sum(
				program[first_index] as usize,
				program[second_index] as usize,
				program,
			);
		} else if op_code == 2 {
			let first_index = i + 1;
			let second_index = i + 2;
			let result_index = program[i + 3] as usize;

			program[result_index] = mult(
				program[first_index] as usize,
				program[second_index] as usize,
				program,
			);
		} else if op_code == 99 {
			cont = false;
		}
		i += 4
	}
}

fn sum(first_position: usize, second_position: usize, op_codes: &mut Vec<i32>) -> i32 {
	op_codes[first_position] + op_codes[second_position]
}

fn mult(first_position: usize, second_position: usize, op_codes: &mut Vec<i32>) -> i32 {
	op_codes[first_position] * op_codes[second_position]
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn simple() {
		let mut v = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
		process_gravity_assist_program(&mut v);
		assert_eq!(v, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}

	#[test]
	fn simple1() {
		let mut v = vec![1, 0, 0, 0, 99];
		process_gravity_assist_program(&mut v);
		assert_eq!(v, [2, 0, 0, 0, 99]);
	}
	#[test]
	fn simple2() {
		let mut v = vec![2, 3, 0, 3, 99];
		process_gravity_assist_program(&mut v);
		assert_eq!(v, [2, 3, 0, 6, 99]);
	}
	#[test]
	fn simple3() {
		let mut v = vec![2, 4, 4, 5, 99, 0];
		process_gravity_assist_program(&mut v);
		assert_eq!(v, [2, 4, 4, 5, 99, 9801]);
	}
}
