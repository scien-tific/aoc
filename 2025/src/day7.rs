use crate::prelude::*;


fn find_start(line: &[u8]) -> usize {
	line.iter()
		.enumerate()
		.find(|(_, b)| **b == b'S')
		.map(|(i, _)| i)
		.unwrap()
}


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut line = Vec::new();
	let mut total = 0;
	
	parser.take_line(b'\n', &mut line)?;
	let mut splits = BitVec::zero(line.len());
	let start = find_start(&line);
	line.clear();
	splits.set(start, true);
	
	while !parser.at_eof()? {
		parser.take_line(b'\n', &mut line)?;
		
		for (i, b) in line.drain(..).enumerate() {
			if b != b'^' || !splits.get(i) {continue;}
			
			splits.set(i, false);
			splits.set(i - 1, true);
			splits.set(i + 1, true);
			total += 1;
		}
	}
	
	Ok(total.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut line = Vec::new();
	
	parser.take_line(b'\n', &mut line)?;
	let mut timelines = vec![0; line.len()];
	let start = find_start(&line);
	line.clear();
	timelines[start] = 1;
	
	while !parser.at_eof()? {
		parser.take_line(b'\n', &mut line)?;
		
		for (i, b) in line.drain(..).enumerate() {
			let count = timelines[i];
			if b != b'^' || count == 0 {continue;}
			
			timelines[i] = 0;
			timelines[i - 1] += count;
			timelines[i + 1] += count;
		}
	}
	
	let sum = timelines.into_iter().sum::<u64>();
	Ok(sum.to_string())
}
