fn main() {
	let mut c = 0;
	// println!("{}", '7' <= '5');
	for n in 256310..732736 {
		if does_number_match_criteria(n) {
			println!("{}", n);
			c += 1;
		}
	}
	println!("{}", c);
}

fn does_number_match_criteria(n: i32) -> bool {
	let number_string: String = n.to_string();
	let mut higher_up_criteria = true;
	let mut double_digit_criteria = false;
	let mut last_char: Option<char> = None;

	for c in number_string.chars() {
		if !double_digit_criteria {
			double_digit_criteria = match last_char {
				Some(l) => l == c,
				None => false,
			};
		}

		higher_up_criteria = match last_char {
			Some(l) => c >= l,

			None => higher_up_criteria,
		};
		if !higher_up_criteria {
			break;
		}

		last_char = Some(c);
	}
	higher_up_criteria && double_digit_criteria
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_path_from_string_down() {
		assert!(does_number_match_criteria(111111));
		assert!(!does_number_match_criteria(732699));
		assert!(!does_number_match_criteria(123789));
	}
}
