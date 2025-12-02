use std::{
	env,
	fs::File,
	time::Instant,
	error::Error,
};


fn main() {
	match run() {
		Err(err) => println!("ERROR: {err}"),
		_ => ()
	}
}


fn run() -> Result<(), Box<dyn Error>> {
	let mut args = env::args().skip(1);
	let name = args.next().ok_or("No solution name!")?;
	let path = args.next().ok_or("No input path!")?;
	let (out, elapsed) = run_solution(&name, &path)?;
	
	println!("{out} (elapsed: {elapsed} µs)");
	Ok(())
}


fn benchmark<E>(func: fn(File) -> Result<String, E>, input: File)
-> Result<(String, u128), Box<dyn Error>>
where E: Error + 'static {
	let timer = Instant::now();
	let result = func(input)?;
	let elapsed = timer.elapsed().as_micros();
	
	Ok((result, elapsed))
}


macro_rules! solutions {
	( $( $module:ident { $( $name:literal => $func:ident ),* } ),* ) => {
		$( mod $module; )*
		
		fn run_solution(name: &str, path: &str) -> Result<(String, u128), Box<dyn Error>> {
			let input = File::open(path)?;
			match name {
				$( $( $name => benchmark($module::$func, input), )* )*
				_ => Err("Invalid solution name!".into())
			}
		}
	}
}


solutions! {
	day1 {
		"d1p1" => part1,
		"d1p2" => part2
	}
}
