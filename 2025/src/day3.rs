use crate::prelude::*;
use std::io::BufReader;


fn parse_line<R: BufRead>(reader: &mut R, buf: &mut Vec<u8>) -> io::Result<bool> {
	buf.clear();
	reader.read_until(b'\n', buf)?;
	buf.pop(); // Ignore newline
	
	Ok(!buf.is_empty())
}


fn max_num(slice: &[u8]) -> (usize, u8) {
	let mut idx = 0;
	let mut max = 0;
	
	for (i, b) in slice.iter().copied().enumerate() {
		// The search can stop if a 9 is found
		if b == b'9' {return (i, b);}
		
		if b > max {
			idx = i;
			max = b;
		}
	}
	
	(idx, max)
}


fn joltage(buf: &[u8], count: usize) -> io::Result<u64> {
	let mut start = 0;
	let mut end = buf.len() - count + 1;
	let mut nums = vec![0; count];
	
	for i in 0..count {
		let (ni, n) = max_num(&buf[start..end]);
		nums[i] = n;
		start += ni + 1;
		end += 1;
	}
	
	SimpleParser::new(&nums[..])?.parse_u64()
}


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut reader = BufReader::new(file);
	let mut buf = Vec::new();
	let mut total = 0;
	
	while parse_line(&mut reader, &mut buf)? {
		total += joltage(&buf, 2)?;
	}
	
	Ok(total.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	let mut reader = BufReader::new(file);
	let mut buf = Vec::new();
	let mut total = 0;
	
	while parse_line(&mut reader, &mut buf)? {
		total += joltage(&buf, 12)?;
	}
	
	Ok(total.to_string())
}
