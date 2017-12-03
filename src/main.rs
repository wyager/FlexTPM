

extern crate core;
// mod lib;
mod mem;
mod prim;
mod multi;
mod evaluator;

use mem::*;
use evaluator::*;
use evaluator::Reg::*;
use evaluator::Instruction::*;
use evaluator::Cond::*;
use evaluator::M2Op::*;

const PROGRAM : &[evaluator::Instruction<u32>] = &[
    Lit{val:4, reg:RA},
    Lit{val:0, reg:R0},
    Lit{val:1, reg:R1},
    Lit{val:100_000_000, reg:R2},
    Um2{op:Add,r1:R0,r2:R1,r3:R0},
    Um2{op:Sub,r1:R0,r2:R2,r3:R3},
    UJump{cond:GtZ,flag:R3,dest:RA},
    Halt
    ];

use core::iter;

fn main() {
    let ram = &mut[0;1024];
    let mut state : State<u32,i32> = evaluator::State::new(ram);
    let instr : evaluator::Instruction<u32> = Lit{val:0, reg:R0};
    let jump = UJump{cond:Always, flag:R0, dest: R0};
    let program2 : Vec<evaluator::Instruction<u32>> = iter::repeat(instr).take(10_000_000).chain(iter::once(jump)).collect();
    let mut memfetch = mem::MemFetch(&program2[0..]);
    // let mut blkfetch : LRU4x64<Instruction<u32>, MemFetch<Instruction<u32>>> = mem::LRU4x64::new(memfetch);
    println!("{:?}",state.eval_instrs(1_00_000_000, &mut memfetch));
    println!("{:?}", core::mem::size_of::<Instruction<u32>>());

    let mut buf : Vec<u8> = Vec::new();
    for instr in PROGRAM {
        instr.write(&mut buf)
    }
    // println!("{:?}", buf);

    let mut read_instrs : [evaluator::Instruction<u32>; 8] = [Halt; 8];
    let res = prim::read_buf(read_instrs.iter_mut(), &mut buf.iter().map(|x| *x));
    println!("{:?}", res);
    println!("{:?}", read_instrs);
}

impl evaluator::WriteSink for Vec<u8> {
    fn write(&mut self, val : u8) -> () {
        self.push(val);
    }
}