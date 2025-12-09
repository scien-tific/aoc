use crate::prelude::*;


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut lines = Vec::new();
	let mut total = 0;
	
	parser.skip_while(|b| b == b' ')?;
	
	while parser.peek()?.is_ascii_digit() {
		let mut line = Vec::new();
		while parser.peek()? != b'\n' {
			line.push(parser.parse_u64()?);
			parser.skip_while(|b| b == b' ')?;
		}
		
		parser.eat(b'\n')?;
		parser.skip_while(|b| b == b' ')?;
		lines.push(line);
	}
	
	let mut idx = 0;
	while !parser.at_eof()? {
		let op = parser.take()?;
		parser.skip_while(|b| b == b' ')?;
		
		match op {
			b'+' =>
			total += lines.iter()
				.map(|l| l[idx])
				.sum::<u64>(),
			
			b'*' =>
			total += lines.iter()
				.map(|l| l[idx])
				.product::<u64>(),
			
			_ => ()
		}
		
		idx += 1;
		if parser.try_eat(b'\n')? {break;}
	}
	
	Ok(total.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut lines = Vec::new();
	let mut total = 0;
	
	while !parser.at_eof()? {
		let mut line = Vec::new();
		parser.take_line(b'\n', &mut line)?;
		lines.push(line);
	}
	
	let ops = lines.pop().unwrap();
	let mut buf = Vec::with_capacity(lines.len());
	let mut nums = Vec::new();
	let mut idx = ops.len() - 1;
	
	loop {
		for ln in lines.iter() {
			let b = ln[idx];
			if !b.is_ascii_digit() {continue;}
			buf.push(b);
		}
		
		let op = ops[idx];
		let num = SimpleParser::new(&buf[..]).parse_u64()?;
		nums.push(num);
		buf.clear();
		
		match op {
			b'+' => total += nums.drain(..).sum::<u64>(),
			b'*' => total += nums.drain(..).product::<u64>(),
			_ => ()
		}
		
		if idx == 0 {break;}
		// Skip extra space after an operator
		else if op != b' ' {idx -= 2;}
		else {idx -= 1;}
	}
	
	Ok(total.to_string())
}
