use crate::prelude::*;


fn parse_ranges<R: BufRead>(parser: &mut SimpleParser<R>) -> io::Result<Vec<(u64, u64)>> {
	let mut ranges = Vec::new();
	
	while !parser.at_eof()? {
		if parser.peek()? == b'\n' {break;}
		
		let min = parser.parse_u64()?;
		parser.eat(b'-')?;
		let max = parser.parse_u64()?;
		parser.eat(b'\n')?;
		ranges.push((min, max));
	}
	
	Ok(ranges)
}


// Sorts ranges and merges overlapping ones
fn fold_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
	let mut vec = Vec::new();
	ranges.sort();
	
	let mut idx = 0;
	while idx < ranges.len() {
		let (start, mut end) = ranges[idx];
		idx += 1;
		
		while idx < ranges.len() {
			let (s, e) = ranges[idx];
			if s > end {break;}
			if e > end {end = e;}
			idx += 1;
		}
		
		vec.push((start, end));
	}
	
	vec
}


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut total = 0;
	
	let mut ranges = parse_ranges(&mut parser)?;
	let folded = fold_ranges(&mut ranges);
	parser.eat(b'\n')?;
	
	while !parser.at_eof()? {
		let id = parser.parse_u64()?;
		parser.eat(b'\n')?;
		
		let mut iter = folded.iter().copied();
		if iter.any(|(s, e)| id >= s && id <= e) {
			total += 1;
		}
	}
	
	Ok(total.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut total = 0;
	let mut ranges = parse_ranges(&mut parser)?;
	
	for (s, e) in fold_ranges(&mut ranges) {
		total += e - s + 1;
	}
	
	Ok(total.to_string())
}
