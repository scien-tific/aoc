use std::io::{self, Read, BufReader, ErrorKind};


#[derive(Debug)]
pub struct SimpleParser<R> {
	reader: R,
	peek: Option<u8>,
}

impl<R: Read> SimpleParser<BufReader<R>> {
	pub fn new_buf(reader: R) -> io::Result<Self> {
		Self::new(BufReader::new(reader))
	}
}

impl<R: Read> SimpleParser<R> {
	pub fn new(reader: R) -> io::Result<Self> {
		let mut parser = Self {reader, peek: None};
		parser.advance()?; // Peek ahead
		
		Ok(parser)
	}
	
	pub fn at_eof(&mut self) -> bool {
		self.peek.is_none()
	}
	
	pub fn peek(&mut self) -> io::Result<u8> {
		self.peek.ok_or(ErrorKind::InvalidData.into())
	}
	
	pub fn take(&mut self) -> io::Result<u8> {
		let prev = self.peek()?;
		self.advance()?;
		Ok(prev)
	}
	
	pub fn take_while<F>(&mut self, mut pred: F, buf: &mut Vec<u8>) -> io::Result<()>
	where F: FnMut(u8) -> bool {
		while let Some(b) = self.peek {
			if !pred(b) {break;}
			buf.push(b);
			self.advance()?;
		}
		
		Ok(())
	}
	
	pub fn eat(&mut self, value: u8) -> io::Result<()> {
		if self.take()? == value {
			Ok(())
		} else {
			Err(ErrorKind::InvalidData.into())
		}
	}
	
	pub fn try_eat(&mut self, value: u8) -> io::Result<bool> {
		let eq = self.peek == Some(value);
		if eq {self.advance()?;}
		Ok(eq)
	}
	
	pub fn parse_u64(&mut self) -> io::Result<u64> {
		let mut num = 0;
		
		while let Some(b) = self.peek {
			if !b.is_ascii_digit() {break;}
			num = num * 10 + u64::from(b - b'0');
			self.advance()?;
		}
		
		Ok(num)
	}
	
	pub fn parse_i64(&mut self) -> io::Result<i64> {
		let sign = if self.try_eat(b'-')? {-1} else {1};
		let num = self.parse_u64()? as i64; // Lossy cast since overflow isn't handled anyway
		Ok(num * sign)
	}
}

impl<R: Read> SimpleParser<R> {
	fn advance(&mut self) -> io::Result<()> {
		let mut buf = [0];
		let count = self.reader.read(&mut buf)?;
		self.peek = (count > 0).then_some(buf[0]);
		
		Ok(())
	}
}
