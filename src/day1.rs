use crate::open_input;
use std::io::{self, BufRead};


fn parse_int(buf: &[u8]) -> i32 {
	let mut exp = 1;
	let mut num = 0;
	
	for c in buf.iter().rev() {
		num += i32::from(c - b'0') * exp;
		exp *= 10;
	}
	
	num
}


pub fn parse_line(buf: &[u8]) -> i32 {
	let end = buf.len() - 1;
	let buf = &buf[..end]; // Ignore newline
	let dir = if buf[0] == b'L' {-1} else {1};
	let num = parse_int(&buf[1..]);
	
	num * dir
}


pub fn part1() -> io::Result<()> {
	let mut file = open_input("day1.txt")?;
	let mut cursor = 50;
	let mut line_buf = Vec::new();
	let mut count = 0;
	
	while file.read_until(b'\n', &mut line_buf)? > 1 {
		let delta = parse_line(&line_buf);
		cursor = (cursor + delta).rem_euclid(100);
		if cursor == 0 {count += 1;}
		line_buf.clear();
	}
	
	println!("Password: {count}");
	Ok(())
}


pub fn part2() -> io::Result<()> {
	let mut file = open_input("day1.txt")?;
	let mut cursor = 50;
	let mut line_buf = Vec::new();
	let mut count = 0;
	
	while file.read_until(b'\n', &mut line_buf)? > 1 {
		let delta = parse_line(&line_buf);
		cursor = (cursor + delta).rem_euclid(100);
		
		if delta > 0 {
			count += (cursor + delta) / 100;
		} else if delta < 0 {
			count += (100 - cursor - delta) / 100;
		} else if cursor == 0 {
			// When delta == 0 && cursor == 0, the turn techincally ends at a 0
			count += 1;
		}
		
		line_buf.clear();
	}
	
	println!("Password: {count}");
	Ok(())
}
