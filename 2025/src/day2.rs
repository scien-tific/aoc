use crate::prelude::*;
use std::collections::HashSet;


fn parse_range<R: Read>(parser: &mut SimpleParser<R>) -> io::Result<(u64, u64)> {
	let start = parser.parse_u64()?;
	parser.eat(b'-')?;
	let end = parser.parse_u64()?;
	Ok((start, end))
}


fn digits(num: u64) -> u32 {
	num.ilog10() + 1
}


fn repeat(num: u64, count: u32) -> u64 {
	let mut result = num;
	let len = digits(num);
	let pow = 10u64.pow(len);
	
	for _ in 1..count {
		result = result * pow + num;
	}
	
	result
}


fn sum_invalid(start: u64, end: u64, count: u32, set: &mut HashSet<u64>) {
	let len = digits(start);
	let mut seed = if len % count == 0 {
		// If digits(start) is divisible by count,
		// the invalid code is seeded with the leading digits of start
		let pow = 10u64.pow(len - len / count);
		start / pow
	} else {
		// When the division isn't even,
		// the closest invalid code >= start is repeated powers of 10
		10u64.pow(len / count)
	};
	
	loop {
		let num = repeat(seed, count);
		if num <= end {
			seed += 1;
			// Disqualify numbers smaller than range start, but don't stop iterating
			if num >= start {set.insert(num);}
		} else {break;}
	}
}


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file)?;
	let mut set = HashSet::new();
	
	while !parser.at_eof() {
		let (start, end) = parse_range(&mut parser)?;
		sum_invalid(start, end, 2, &mut set);
		if !parser.try_eat(b',')? {break;}
	}
	
	let sum = set.into_iter().sum::<u64>();
	Ok(sum.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	// Only cases with a prime number of repeats need to be considered,
	// since anything divisible into 4 repeated patterns can also be represented as just 2, etc.
	const PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
	
	let mut parser = SimpleParser::new_buf(file)?;
	let mut set = HashSet::new();
	
	while !parser.at_eof() {
		let (start, end) = parse_range(&mut parser)?;
		let len = digits(end);
		
		for c in PRIMES.into_iter().take_while(|p| *p <= len) {
			sum_invalid(start, end, c, &mut set);
		}
		
		if !parser.try_eat(b',')? {break;}
	}
	
	let sum = set.into_iter().sum::<u64>();
	Ok(sum.to_string())
}
