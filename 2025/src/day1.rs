use crate::prelude::*;


fn parse_line<R: BufRead>(parser: &mut SimpleParser<R>) -> io::Result<i64> {
	let dir = if parser.take()? == b'L' {-1} else {1};
	let delta = parser.parse_i64()? * dir;
	parser.eat(b'\n')?;
	
	Ok(delta)
}


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut cursor = 50;
	let mut count = 0;
	
	while !parser.at_eof()? {
		let delta = parse_line(&mut parser)?;
		cursor = (cursor + delta).rem_euclid(100);
		if cursor == 0 {count += 1;}
	}
	
	Ok(count.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut cursor = 50;
	let mut count = 0;
	
	while !parser.at_eof()? {
		let delta = parse_line(&mut parser)?;
		
		count += if delta > 0 {
			(cursor + delta) / 100
		} else if delta < 0 {
			if cursor == 0 {
				// When the turns starts at 0, the initial 0 doesn't count
				-delta / 100
			} else {
				(100 - cursor - delta) / 100
			}
		} else if cursor == 0 {
			1 // When delta == 0 && cursor == 0, the turn techincally ends at a 0
		} else {0};
		
		cursor = (cursor + delta).rem_euclid(100);
	}
	
	Ok(count.to_string())
}
