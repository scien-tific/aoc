use crate::prelude::*;


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let grid = parser.parse_bitgrid(|b| b == b'@', b'\n')?;
	
	todo!();
}


pub fn part2(file: File) -> Result<String, AocErr> {
	todo!();
}
