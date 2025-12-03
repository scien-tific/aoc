use crate::prelude::*;


fn parse_range<R: Read>(parser: &mut SimpleParser<R>) -> io::Result<(u64, u64)> {
	let start = parser.parse_u64()?;
	parser.eat(b'-')?;
	let end = parser.parse_u64()?;
	Ok((start, end))
}


pub fn part1(file: File) -> io::Result<String> {
	let mut parser = SimpleParser::new_buf(file)?;
	let mut total = 0;
	
	while !parser.at_eof() {
		let (start, end) = parse_range(&mut parser)?;
		let digits = start.ilog10() + 1;
		let pow = 10u64.pow(digits / 2);
		
		let mut half = if digits % 2 != 0 {
			// If the amount of digits is uneven,
			// the smallest invalid id that could be in the range is a repeated power of 10
			pow
		} else {
			// With an even number of digits,
			// the smallest invalid id that could be in the range is the upper half of the start repeated
			start / pow
		};
		
		loop {
			let digits = half.ilog10() + 1;
			let mut num = half * 10u64.pow(digits) + half;
			
			if num <= end {
				half += 1;
				// Disqualify numbers smaller than range start, but don't stop iterating
				if num >= start {total += num;}
			} else {break;}
		}
		
		if !parser.try_eat(b',')? {break;}
	}
	
	Ok(total.to_string())
}


pub fn part2(file: File) -> io::Result<String> {
	let mut parser = SimpleParser::new_buf(file)?;
	todo!();
}
