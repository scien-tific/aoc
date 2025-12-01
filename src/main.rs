mod day1;

use std::{
	env,
	io::{self, BufReader},
	path::Path,
	fs::File,
};


const INPUT_DIR: &'static str = "input/";


pub fn open_input<P>(name: P) -> io::Result<BufReader<File>>
where P: AsRef<Path> {
	let path = Path::new(INPUT_DIR).join(name.as_ref());
	Ok(BufReader::new(File::open(path)?))
}


fn main() -> io::Result<()> {
	let mut args = env::args().skip(1);
	let name = args.next().expect("No solution name!");
	
	match name.to_ascii_lowercase().as_str() {
		"d1p1" => day1::part1(),
		"d1p2" => day1::part2(),
		_ => panic!("Mangled solution name!")
	}
}
