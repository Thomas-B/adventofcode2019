use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;


fn main() {
	let file = File::open("input.txt");
	let file = match file {
		Ok(file) => file,
		Err(error) => {
			panic!("Couldn't open file got error: {:?}", error);
		}
	};
	let reader = BufReader::new(file);
	let mut paths: Vec<HashSet<(i32,i32)>> = Vec::new();

	for line in reader.lines() {
		let line = match line {
			Ok(line) => line,
			Err(error) => panic!("Couldn't read line got error: {:?}", error),
		};

		println!("{:?}",line);
		let directions = line.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
		paths.push(get_path_coords(directions));
	}
	let first_path = &paths[0];
	let second_path = &paths[1];

	println!("{:?}", get_closest_intersection_distance(first_path, second_path));
}

fn get_closest_intersection_distance(first_path: &HashSet<(i32, i32)>, second_path: &HashSet<(i32, i32)>) -> i32 {
		let intersection = first_path.intersection(&second_path);


	let mut min_distance = -1;
	for i in intersection {
		let distance = get_distance_from_origin(*i);

		if min_distance == -1 || distance < min_distance  {
			min_distance = distance;
		}
	}
	min_distance
}

fn get_distance_from_origin(coord: (i32, i32)) -> i32 {
	let x = coord.0;
	let y = coord.1;
	x.abs() +y.abs()
}

fn get_path_coords(directions: Vec<String>) -> HashSet<(i32,i32)> {
	let mut point = Point { x:0, y:0};
	let mut all_coords: HashSet<(i32,i32)> =  HashSet::new();

	for  d  in directions {
		let path = match Path::get_path_from_string(&d) {
			Some(p) => p,
			None => panic!("Couldn't get path from string: {}", d),
		};
		let generated_coords = point.draw_path(path);
		for g in generated_coords {
			all_coords.insert(g);
		}

	}

	all_coords
}

struct Point {
	x: i32,
	y: i32
}

impl Point {
	fn draw_path(&mut self,path: Path) -> Vec<(i32, i32)> {
		match path.direction {
			Direction::Left => self.go_left(path.length),
			Direction::Up=> self.go_up(path.length),
			Direction::Right=> self.go_right(path.length),
			Direction::Down=> self.go_down(path.length),
		}
	}

	fn go_left (&mut self, length: i32) -> Vec<(i32, i32)> {
		let mut coords: Vec<(i32, i32)> = vec!();
		for _ in 0..length {
				self.x -= 1;
				coords.push(self.to_tuple());
		}
		coords
	}

	fn go_up (&mut self, length: i32)  -> Vec<(i32, i32)> {
		let mut coords: Vec<(i32, i32)> = vec!();
		for _ in 0..length {
				self.y += 1;
				coords.push(self.to_tuple());
		}
		coords
	}

	fn go_right (&mut self, length: i32)  -> Vec<(i32, i32)> {
		let mut coords: Vec<(i32, i32)> = vec!();
		for _ in 0..length {
				self.x += 1;
				coords.push(self.to_tuple());
		}
		coords
	}

	fn go_down(&mut self, length: i32)  -> Vec<(i32, i32)> {
		let mut coords: Vec<(i32, i32)> = vec!();
		for _ in 0..length {
				self.y -= 1;
				coords.push(self.to_tuple());
		}
		coords
	}
	fn to_tuple(&self) -> (i32,i32) {
		(self.x, self.y)
	}
}

enum Direction {
	Left,
	Up,
	Right,
	Down
}

struct Path {
	direction: Direction,
	length: i32,
}

impl Path {
	fn get_path_from_string(path: &str)-> Option<Path> {
		let mut char_iter = path.chars();
		let direction_string = char_iter.next()?;
		let length  = char_iter.collect::<String>().parse::<i32>().unwrap();

		let direction = match direction_string.to_string().as_ref() {
			"R" => Direction::Right,
			"U" => Direction::Up,
			"L" => Direction::Left,
			"D" => Direction::Down,
			_ => panic!("direction not handled {}", direction_string)
		};
		Some(Path {direction, length})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn complete() {
		let first_path = get_path_coords(vec![String::from("R75"),String::from("D30"),String::from("R83"),String::from("U83"),String::from("L12"),String::from("D49"),String::from("R71"),String::from("U7"),String::from("L72")]);
		let second_path = get_path_coords(vec![String::from("U62"),String::from("R66"),String::from("U55"),String::from("R34"),String::from("D71"),String::from("R55"),String::from("D58"),String::from("R83")]);
		println!("{:?}",first_path);
		// println!("{:?}", second_path);
		let min_distance = get_closest_intersection_distance(&first_path, &second_path);
		assert_eq!(min_distance, 159)
	}

	#[test]
	fn complete_second() {
		let first_path = get_path_coords(vec!["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"].into_iter().map(|s| String::from(s)).collect());
		let second_path = get_path_coords(vec!["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"].into_iter().map(|s| String::from(s)).collect());
		println!("{:?}",first_path);
		// println!("{:?}", second_path);
		let min_distance = get_closest_intersection_distance(&first_path, &second_path);
		assert_eq!(min_distance, 135)
	}

	#[test]
	fn go_down() {
		let mut p = Point {x:0,y:0};
		p.go_down(1);
		assert_eq!(p.to_tuple(), (0,-1));
		// assert_eq!(v, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}


	#[test]
	fn go_up() {
		let mut p = Point {x:0,y:0};
		p.go_up(1);
		assert_eq!(p.to_tuple(), (0,1));
		// assert_eq!(v, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}

	#[test]
	fn go_right() {
		let mut p = Point {x:0,y:0};
		p.go_right(1);
		assert_eq!(p.to_tuple(), (1,0));
		// assert_eq!(v, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}


	#[test]
	fn go_left() {
		let mut p = Point {x:0,y:0};
		p.go_left(1);
		assert_eq!(p.to_tuple(), (-1, 0));
		// assert_eq!(v, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
	}


	#[test]
	fn get_path_from_string_right() {
		let dir_string = String::from("R75");
		let path = match Path::get_path_from_string(&dir_string){
			Some(path) => path,
			None => panic!("got error"),
		};

		// crappy way to test enum value/variant I don't know
		assert!(if let Direction::Right = path.direction { true } else { false });
		assert_eq!(path.length, 75);
	}


	#[test]
	fn get_path_from_string_up() {
		let dir_string = String::from("U3456");
		let path = match Path::get_path_from_string(&dir_string){
			Some(path) => path,
			None => panic!("got error"),
		};

		assert!(if let Direction::Up = path.direction { true } else { false });
		assert_eq!(path.length, 3456);
	}

	#[test]
	fn get_path_from_string_left() {
		let dir_string = String::from("L34573734");
		let path = match Path::get_path_from_string(&dir_string){
			Some(path) => path,
			None => panic!("got error"),
		};


		assert!(if let Direction::Left = path.direction { true } else { false });
		assert_eq!(path.length, 34_573_734);
	}

	#[test]
	fn get_path_from_string_down() {
		let dir_string = String::from("D1");
		let path = match Path::get_path_from_string(&dir_string){
			Some(path) => path,
			None => panic!("got error"),
		};


		assert!(if let Direction::Down = path.direction { true } else { false });
		assert_eq!(path.length, 1);
	}
}
