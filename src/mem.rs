use evaluator::*;

// Just a wrapper around an array containing a bunch of instructions.
// Good for testing stuff on desktop and very simple/fast,
// but probably not what you want on very RAM-constrained devices.

pub struct MemFetch<'a, T : 'a>(pub &'a [T]);

impl<'a,Ptr:Prim,T:Copy> Fetcher<Ptr,T> for MemFetch<'a, T> {
    fn fetch(&mut self, ptr : Ptr) -> Option<T> {
        let &mut MemFetch(arr) = self;
        arr.get(ptr.to_usize()).map(|t| *t)
    }
}

use core::cmp::min;

impl<'a,Ptr:Prim,T:Copy + Default> Load64<Ptr,T> for MemFetch<'a,T> {
    fn load(&mut self, ptr:Ptr) -> Option<([T; 64], usize)> {
        let &mut MemFetch(arr) = self;
        let start_ptr = (ptr << 6).to_usize();
        if start_ptr >= arr.len() {
            None
        } else {
            let end_ptr = min(arr.len(), start_ptr + 64);
            let copy_len = end_ptr - start_ptr;
            let mut result = [T::default(); 64];
            result[0..copy_len].copy_from_slice(&arr[start_ptr.. start_ptr + copy_len]);
            Some((result,copy_len))
        }
    }
}

// Keep a 4x64 instruction LRU cache. Good for loading instructions off ROM.
// If you're using 32-bit arithmetic, this takes a bit over 3KiB of RAM to hold 
// those 256 instructions unpacked in RAM.

#[derive(Clone,Copy)]
struct Line64<T> {
    last_use : usize,
    length : usize,
    loc : usize,
    values : [T;64]
}

impl<T : Copy> Line64<T> {
    fn get(&self, ptr : usize) -> Option<T> {
        let offset = ptr - self.loc;
        if ptr >= self.loc && offset < self.length {
            Some(self.values[offset])
        } else {
            None
        }
    }
    fn get_and_set(&mut self, ptr : usize, counter : usize) -> Option<T> {
        match self.get(ptr) {
            None => None,
            Some(t) => {
                self.last_use = counter;
                Some(t)
            }
        }
    }
}

pub struct LRU4x64<T,L> {
    counter : usize,
    lines : [Option<Line64<T>>; 4],
    loader : L
}

impl<T : Copy,L> LRU4x64<T,L> {
    pub fn new(loader : L) -> Self {
        LRU4x64 {
            counter : 0,
            lines : [None; 4],
            loader : loader
        }
    }
}

fn next_free<'a, T>(lines: &'a mut [Option<Line64<T>>; 4]) -> &'a mut Option<Line64<T>> {
    let free = lines.iter_mut()
        .min_by_key(|line| match *line {
            &mut None => 0, 
            &mut Some(ref line) => line.last_use});
    match free {
        Some(line) => return line,
        None => panic!("Impossible")
    }
}

pub trait Load64<Ptr,T> {
    fn load(&mut self, Ptr) -> Option<([T; 64], usize)>;
}
use core::fmt::Debug;
impl<Ptr : Prim, T : Copy + Debug, L:Load64<Ptr,T>> Fetcher<Ptr,T> for LRU4x64<T,L> {
    fn fetch(&mut self, ptr : Ptr) -> Option<T> {
        self.counter += 1;
        for line in self.lines.iter_mut() {
            match line {
                &mut Some(ref mut line) => {
                    match line.get_and_set(ptr.to_usize(),self.counter) {
                        Some(t) => return Some(t),
                        None => ()
                    }
                }
                &mut None => ()
            }
        }
        let free_line = next_free(&mut self.lines);
        let block_ptr = ptr >> 6;
        let last_use = self.counter;
        match self.loader.load(block_ptr) {
            None => None, // The requested block was out of bounds
            Some((values,length)) => {
                let line = Line64 { last_use, length, loc:(block_ptr << 6).to_usize(), values };
                let result = line.get(ptr.to_usize());
                *free_line = Some(line);
                return result;
            }
        }
    }
}
