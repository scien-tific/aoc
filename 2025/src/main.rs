pub mod common;
pub mod prelude;

use std::{
	env, io, fmt,
	fs::File,
	time::Instant,
	num::ParseIntError,
	error::Error,
};


pub type Solution = fn(File) -> Result<String, AocErr>;


#[derive(Debug)]
pub enum AocErr {
	NoArg(&'static str),
	NoSolution,
	ParseIntErr(ParseIntError),
	IoErr(io::Error),
}

impl fmt::Display for AocErr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NoArg(arg) => write!(f, "Missing argument: {arg}"),
			Self::NoSolution => write!(f, "Solution not found!"),
			Self::ParseIntErr(err) => err.fmt(f),
			Self::IoErr(err) => write!(f, "IO error: {err}"),
		}
	}
}

impl From<io::Error> for AocErr {
	fn from(err: io::Error) -> Self {
		Self::IoErr(err)
	}
}

impl From<ParseIntError> for AocErr {
	fn from(err: ParseIntError) -> Self {
		Self::ParseIntErr(err)
	}
}

impl Error for AocErr {}


fn main() {
	match run() {
		Err(err) => println!("[ERROR] {err}"),
		_ => ()
	}
}


fn run() -> Result<(), AocErr> {
	let mut args = env::args().skip(1);
	let name = args.next().ok_or(AocErr::NoArg("NAME"))?;
	let path = args.next().ok_or(AocErr::NoArg("PATH"))?;
	let func = get_solution(&name).ok_or(AocErr::NoSolution)?;
	
	if let Some(count_str) = args.next() {
		let count = count_str.parse()?;
		if count == 0 {return Ok(());}
		
		let (best, worst, avg) = benchmark(func, count, &path)?;
		println!("Best: {best} µs, Worst: {worst} µs, Avg: {avg} µs");
	} else {
		let input = File::open(&path)?;
		let (out, elapsed) = run_solution(func, input)?;
		println!("{out} (elapsed: {elapsed} µs)");
	}
	
	Ok(())
}


fn benchmark(func: Solution, count: u32, path: &str) -> Result<(u128, u128, u128), AocErr> {
	let mut worst = u128::MIN;
	let mut best = u128::MAX;
	let mut sum = 0;
	
	for _ in 0..count {
		let input = File::open(&path)?;
		let (_, elapsed) = run_solution(func, input)?;
		
		sum += elapsed;
		if elapsed < best {best = elapsed;}
		if elapsed > worst {worst = elapsed;}
	}
	
	let avg = sum / u128::from(count);
	Ok((best, worst, avg))
}


fn run_solution(func: Solution, input: File) -> Result<(String, u128), AocErr> {
	let timer = Instant::now();
	let result = func(input)?;
	let elapsed = timer.elapsed().as_micros();
	
	Ok((result, elapsed))
}


macro_rules! solutions {
	( $( $module:ident { $( $name:literal => $func:ident ),* } ),* ) => {
		$( mod $module; )*
		
		fn get_solution(name: &str) -> Option<Solution> {
			match name {
				$( $( $name => Some($module::$func), )* )*
				_ => None
			}
		}
	}
}


solutions! {
	day1 {
		"d1p1" => part1,
		"d1p2" => part2
	},
	
	day2 {
		"d2p1" => part1,
		"d2p2" => part2
	},
	
	day3 {
		"d3p1" => part1,
		"d3p2" => part2
	}
}
