use crate::prelude::*;


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
	
	SimpleParser::new(&nums[..]).parse_u64()
}


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut buf = Vec::new();
	let mut total = 0;
	
	while parser.take_line(b'\n', &mut buf)? > 0 {
		total += joltage(&buf, 2)?;
		buf.clear();
	}
	
	Ok(total.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut buf = Vec::new();
	let mut total = 0;
	
	while parser.take_line(b'\n', &mut buf)? > 0 {
		total += joltage(&buf, 12)?;
		buf.clear();
	}
	
	Ok(total.to_string())
}
