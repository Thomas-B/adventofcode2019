use std::collections::HashMap;

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
	let mut last_char: Option<char> = None;
	let mut char_occurences = HashMap::new();

	for c in number_string.chars() {
		higher_up_criteria = match last_char {
			Some(l) => c >= l,

			None => higher_up_criteria,
		};

		if !higher_up_criteria {
			break;
		}

		let count = char_occurences.entry(c).or_insert(0);
		*count += 1;
		last_char = Some(c);
	}
	higher_up_criteria && char_occurences.values().any(|count| *count == 2)
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part_two() {
		assert!(does_number_match_criteria(112233));
		assert!(!does_number_match_criteria(123444));
		assert!(!does_number_match_criteria(12222));
	}
}
