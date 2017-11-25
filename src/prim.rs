use lib::*;


impl Prim for u64 {
    fn to_usize(&self) -> usize {*self as usize}
    fn zero() -> u64 {0}
    fn one() -> u64 {1}
}

impl Prim for i64 {
    fn to_usize(&self) -> usize {*self as usize}
    fn zero() -> i64 {0}
    fn one() -> i64 {1}
}


impl Prim for u32 {
    fn to_usize(&self) -> usize {*self as usize}
    fn zero() -> u32 {0}
    fn one() -> u32 {1}
}

impl Prim for i32 {
    fn to_usize(&self) -> usize {*self as usize}
    fn zero() -> i32 {0}
    fn one() -> i32 {1}
}

impl Compl<i64> for u64 {
    fn compl(&self) -> i64 {*self as i64}
}

impl Compl<u64> for i64 {
    fn compl(&self) -> u64 {*self as u64}
}

impl Compl<i32> for u32 {
    fn compl(&self) -> i32 {*self as i32}
}

impl Compl<u32> for i32 {
    fn compl(&self) -> u32 {*self as u32}
}

use core::slice::IterMut;

pub fn read_buf<It:Iterator<Item=u8>, T : Read> (dst : IterMut<T>, src :&mut It) -> Result<(),()> {
    for t in dst {
        match T::read(src) {
            Err(()) => return Err(()),
            Ok(t2) => *t = t2
        }
    }
    return Ok(())
}

use core::iter::IntoIterator;

pub fn write_iter<T:IntoIterator<Item=X>, X: Write, Sink : WriteSink>(t:T, sink:&mut Sink) {
    for item in t.into_iter() {
        item.write(sink);
    }
}

impl Read for u8 {
    fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<u8,()> {
        match it.next() {
            None => Err(()),
            Some(u) => Ok(u)
        }
    }
}

impl Read for u32 {
    fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<u32,()> {
        let mut buf : [u8; 4] = [0;4];
        read_buf(buf.iter_mut(), it).map(|()| buf.iter().fold(0, |acc,x| acc * 256 + (*x as u32)))
    }
}

impl Read for i32 {
     fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<i32,()> {
        u32::read(it).map(|x| x as i32)
    }
}

impl Read for u64 {
    fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<u64,()> {
        let mut buf : [u8; 8] = [0;8];
        read_buf(buf.iter_mut(), it).map(|()| buf.iter().fold(0, |acc,x| acc * 256 + (*x as u64)))
    }
}

impl Read for i64 {
     fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<i64,()> {
        u64::read(it).map(|x| x as i64)
    }
}


impl Write for u32 {
    fn write<Sink : WriteSink>(&self, sink: &mut Sink) -> () {
        sink.write((*self >> 24) as u8);
        sink.write((*self >> 16) as u8);
        sink.write((*self >> 8) as u8);
        sink.write(*self as u8);
    }
}

impl Write for i32 {
    fn write<Sink : WriteSink>(&self, sink: &mut Sink) -> () {
        (*self as u32).write(sink)
    }
}