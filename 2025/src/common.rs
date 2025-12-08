use std::io::{
	self,
	Read,
	BufRead,
	BufReader,
	ErrorKind,
};


#[derive(Debug)]
pub struct SimpleParser<R> {
	reader: R,
}

impl<R: Read> SimpleParser<BufReader<R>> {
	pub fn new_buf(reader: R) -> Self {
		Self::new(BufReader::new(reader))
	}
}

impl<R: BufRead> SimpleParser<R> {
	pub fn new(reader: R) -> Self {
		Self {reader}
	}
	
	pub fn at_eof(&mut self) -> io::Result<bool> {
		let buf = self.reader.fill_buf()?;
		Ok(buf.is_empty())
	}
	
	pub fn peek(&mut self) -> io::Result<u8> {
		let buf = self.reader.fill_buf()?;
		buf.first().copied().ok_or(ErrorKind::InvalidData.into())
	}
	
	pub fn take(&mut self) -> io::Result<u8> {
		let peek = self.peek()?;
		self.reader.consume(1);
		
		Ok(peek)
	}
	
	pub fn take_while<F>(&mut self, mut pred: F, buf: &mut Vec<u8>) -> io::Result<usize>
	where F: FnMut(u8) -> bool {
		let prev_len = buf.len();
		
		loop {
			let rbuf = self.reader.fill_buf()?;
			if rbuf.is_empty() {break;}
			
			// Find index of first non-match
			let idx = rbuf.iter()
				.enumerate()
				.find(|(_, b)| !pred(**b))
				.map(|(i, _)| i);
			
			if let Some(end) = idx {
				buf.extend(&rbuf[..end]);
				self.reader.consume(end);
				break;
			} else {
				let len = rbuf.len();
				buf.extend(rbuf);
				self.reader.consume(len);
			}
		}
		
		Ok(buf.len() - prev_len)
	}
	
	pub fn take_line(&mut self, sep: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
		let prev_len = buf.len();
		self.reader.read_until(sep, buf)?;
		
		// Get rid of that pesky separator
		if buf.last() == Some(&sep) {
			buf.pop();
		}
		
		Ok(buf.len() - prev_len)
	}
	
	pub fn eat(&mut self, value: u8) -> io::Result<()> {
		if self.take()? == value {
			Ok(())
		} else {
			Err(ErrorKind::InvalidData.into())
		}
	}
	
	pub fn try_eat(&mut self, value: u8) -> io::Result<bool> {
		let eq = self.peek()? == value;
		if eq {self.reader.consume(1);}
		
		Ok(eq)
	}
	
	pub fn parse_u64(&mut self) -> io::Result<u64> {
		let mut num = 0;
		
		while !self.at_eof()? {
			let b = self.peek()?;
			if !b.is_ascii_digit() {break;}
			num = num * 10 + u64::from(b - b'0');
			self.reader.consume(1);
		}
		
		Ok(num)
	}
	
	pub fn parse_i64(&mut self) -> io::Result<i64> {
		let sign = if self.try_eat(b'-')? {-1} else {1};
		let num = self.parse_u64()? as i64; // Lossy cast since overflow isn't handled anyway
		
		Ok(num * sign)
	}
	
	pub fn parse_bitgrid<F>(&mut self, mut map: F, sep: u8) -> io::Result<BitGrid>
	where F: FnMut(u8) -> bool {
		let mut chunks = Vec::new();
		let mut buf = Vec::new();
		
		let width = self.take_line(sep, &mut buf)?;
		let chunk_width = width.div_ceil(64);
		
		loop {
			let start = chunks.len();
			chunks.resize(start + chunk_width, 0);
			
			for i in 0..buf.len() {
				if map(buf[i]) {
					let chunk = start + i / 64;
					let bit = i % 64;
					chunks[chunk] |= 1 << bit;
				}
			}
			
			buf.clear();
			if self.take_line(sep, &mut buf)? == 0 {break;}
		}
		
		Ok(BitGrid::from_chunks(chunks, width))
	}
}


#[derive(Debug, Clone)]
pub struct BitGrid {
	chunks: Vec<u64>,
	width: usize,
}

impl BitGrid {
	pub fn from_chunks(chunks: Vec<u64>, width: usize) -> Self {
		Self {chunks, width}
	}
	
	pub fn new(width: usize, height: usize) -> Self {
		let size = width.div_ceil(64) * height;
		Self {chunks: vec![0; size], width}
	}
	
	pub fn width(&self) -> usize {
		self.width
	}
	
	pub fn height(&self) -> usize {
		self.chunks.len() / self.width.div_ceil(64)
	}
	
	pub fn get(&self, x: isize, y: isize) -> bool {
		match self.pos_idx(x, y) {
			Some((idx, bit)) => (self.chunks[idx] >> bit) & 1 == 1,
			None => false,
		}
	}
	
	pub fn set(&mut self, x: isize, y: isize, value: bool) {
		let (idx, bit) = match self.pos_idx(x, y) {
			Some(val) => val,
			None => return,
		};
		
		let chunk = self.chunks[idx];
		let shift = 1 << bit;
		
		if value {
			self.chunks[idx] = chunk | shift;
		} else {
			self.chunks[idx] = chunk ^ (chunk & shift);
		}
	}
}

impl BitGrid {
	fn pos_idx(&self, x: isize, y: isize) -> Option<(usize, usize)> {
		if x < 0 || y < 0 {return None;}
		
		let x = x as usize;
		let y = y as usize;
		
		let chunk_width = self.width.div_ceil(64);
		let height = self.chunks.len() / chunk_width;
		
		if x >= self.width || y >= height {return None;}
		
		let idx = x / 64 + y * chunk_width;
		let bit = x % 64;
		
		Some((idx, bit))
	}
}

