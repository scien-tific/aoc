use crate::prelude::*;


fn count_neighbors(x: isize, y: isize, grid: &BitGrid) -> usize {
	let positions = [
		(x - 1, y - 1),
		(x    , y - 1),
		(x + 1, y - 1),
		(x - 1, y    ),
		(x + 1, y    ),
		(x - 1, y + 1),
		(x    , y + 1),
		(x + 1, y + 1),
	];
	
	positions.into_iter()
		.filter(|(px, py)| grid.get(*px, *py))
		.count()
}


pub fn part1(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let grid = parser.parse_bitgrid(|b| b == b'@', b'\n')?;
	let width = grid.width() as isize;
	let height = grid.height() as isize;
	let mut total = 0;
	
	for y in 0..height {
	for x in 0..width {
		if !grid.get(x, y) {continue;}
		
		if count_neighbors(x, y, &grid) < 4 {
			total += 1;
		}
	}}
	
	Ok(total.to_string())
}


pub fn part2(file: File) -> Result<String, AocErr> {
	let mut parser = SimpleParser::new_buf(file);
	let mut grid = parser.parse_bitgrid(|b| b == b'@', b'\n')?;
	let width = grid.width() as isize;
	let height = grid.height() as isize;
	let mut remove = Vec::new();
	let mut total = 0;
	
	loop {
		for y in 0..height {
		for x in 0..width {
			if !grid.get(x, y) {continue;}
			
			if count_neighbors(x, y, &grid) < 4 {
				remove.push((x, y));
			}
		}}
		
		if remove.is_empty() {break;}
		total += remove.len();
		
		for (x, y) in remove.drain(..) {
			grid.set(x, y, false);
		}
	}
	
	Ok(total.to_string())
}
