use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt");
    let input = match input {
        Ok(i) => i,
        Err(error) => panic!("Couldn't read file got error: {:?}", error),
    };

    let op_codes: Vec<i32> = input
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

    process_gravity_assist_program(&mut op_codes.clone());
}

fn process_gravity_assist_program(program: &mut Vec<i32>) {
    let mut cont = true;
    let mut i = 0;
    let input: i32 = 5;
    let mut output: i32 = 0;

    while cont {
        let first_param: i32 = program[i];
        let first_param_string = format!("{:0>4}", first_param.to_string());
        let mut c = first_param_string.chars();
        let second_parameter_mode = match c.next() {
            Some(t) => t == '1',
            None => panic!("couldnt get 2nd parameter mode"),
        };
        let first_parameter_mode = match c.next() {
            Some(t) => t == '1',
            None => panic!("couldnt get 2nd parameter mode"),
        };
        let op_code = i32::from_str_radix(c.as_str(), 10).unwrap_or(11);
        i += match op_code {
            1 => sum(program, i, first_parameter_mode, second_parameter_mode),
            2 => mult(program, i, first_parameter_mode, second_parameter_mode),
            3 => store_input(program, i, input),
            4 => get_output(program, i, &mut output),
            5 => jump_if_true(program, &mut i, first_parameter_mode, second_parameter_mode),
            6 => jump_if_false(program, &mut i, first_parameter_mode, second_parameter_mode),
            7 => less_than(program, i, first_parameter_mode, second_parameter_mode),
            8 => equals(program, i, first_parameter_mode, second_parameter_mode),
            99 => {
                cont = false;
                println!("{}", output);
                0
            }
            _ => panic!("unhandled value {}", op_code),
        };
    }
}

fn jump_if_true(
    program: &mut Vec<i32>,
    index: &mut usize,
    first_parameter_mode: bool,
    second_parameter_mode: bool,
) -> usize {
    let first_value = if first_parameter_mode {
        program[*index + 1]
    } else {
        program[program[*index + 1] as usize]
    };
    let second_value = if second_parameter_mode {
        program[*index + 2]
    } else {
        program[program[*index + 2] as usize]
    };
    if first_value != 0 {
        *index = second_value as usize;
        return 0;
    }
    3
}
fn jump_if_false(
    program: &mut Vec<i32>,
    index: &mut usize,
    first_parameter_mode: bool,
    second_parameter_mode: bool,
) -> usize {
    let first_value = if first_parameter_mode {
        program[*index + 1]
    } else {
        program[program[*index + 1] as usize]
    };
    let second_value = if second_parameter_mode {
        program[*index + 2]
    } else {
        program[program[*index + 2] as usize]
    };
    if first_value == 0 {
        *index = second_value as usize;
        return 0;
    }
    3
}

fn less_than(
    program: &mut Vec<i32>,
    index: usize,
    first_parameter_mode: bool,
    second_parameter_mode: bool,
) -> usize {
    let first_value = if first_parameter_mode {
        program[index + 1]
    } else {
        program[program[index + 1] as usize]
    };
    let second_value = if second_parameter_mode {
        program[index + 2]
    } else {
        program[program[index + 2] as usize]
    };
    let result_index = program[index + 3] as usize;

    if first_value < second_value {
        program[result_index] = 1;
    } else {
        program[result_index] = 0;
    }
    4
}

fn equals(
    program: &mut Vec<i32>,
    index: usize,
    first_parameter_mode: bool,
    second_parameter_mode: bool,
) -> usize {
    let first_value = if first_parameter_mode {
        program[index + 1]
    } else {
        program[program[index + 1] as usize]
    };
    let second_value = if second_parameter_mode {
        program[index + 2]
    } else {
        program[program[index + 2] as usize]
    };
    let result_index = program[index + 3] as usize;

    if first_value == second_value {
        program[result_index] = 1;
    } else {
        program[result_index] = 0;
    }
    4
}

fn store_input(program: &mut Vec<i32>, index: usize, input: i32) -> usize {
    let index = program[index + 1] as usize;
    program[index] = input;
    2
}

fn get_output(program: &mut Vec<i32>, index: usize, output: &mut i32) -> usize {
    *output = program[program[index + 1] as usize];
    2
}

fn sum(
    program: &mut Vec<i32>,
    index: usize,
    first_parameter_mode: bool,
    second_parameter_mode: bool,
) -> usize {
    let first_value = if first_parameter_mode {
        program[index + 1]
    } else {
        program[program[index + 1] as usize]
    };
    let second_value = if second_parameter_mode {
        program[index + 2]
    } else {
        program[program[index + 2] as usize]
    };
    let result_index = program[index + 3] as usize;

    program[result_index] = first_value + second_value;
    4
}

fn mult(
    program: &mut Vec<i32>,
    index: usize,
    first_parameter_mode: bool,
    second_parameter_mode: bool,
) -> usize {
    let first_value = if first_parameter_mode {
        program[index + 1]
    } else {
        program[program[index + 1] as usize]
    };
    let second_value = if second_parameter_mode {
        program[index + 2]
    } else {
        program[program[index + 2] as usize]
    };
    let result_index = program[index + 3] as usize;

    program[result_index] = first_value * second_value;
    4
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
        println!("{:?}", v);
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
