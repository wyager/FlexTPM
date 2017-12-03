

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
use evaluator::MathInstr::*;
use evaluator::Cond::*;
use evaluator::M2Op::*;

const PROGRAM : &[evaluator::Instruction<u32,i32>] = &[
    Unsigned{instr:Lit{val:4, reg:RA}},
    Unsigned{instr:Lit{val:0, reg:R0}},
    Unsigned{instr:Lit{val:1, reg:R1}},
    Unsigned{instr:Lit{val:100_000_000, reg:R2}},
    Unsigned{instr:Math2{op:Add,r1:R0,r2:R1,r3:R0}},
    Unsigned{instr:Math2{op:Sub,r1:R0,r2:R2,r3:R3}},
    Unsigned{instr:Jump{cond:GtZ,flag:R3,dest:RA}},
    Halt
    ];

use core::iter;

fn main() {
    let ram = &mut[0;1024];
    let mut state : State<u32,i32> = evaluator::State::new(ram);
    let instr : evaluator::Instruction<u32,i32> = Unsigned{instr:Lit{val:0, reg:R0}};
    let jump = Unsigned{instr:Jump{cond:Always, flag:R0, dest: R0}};
    let program2 : Vec<evaluator::Instruction<u32,i32>> = iter::repeat(instr).take(10_000_000).chain(iter::once(jump)).collect();
    let mut memfetch = mem::MemFetch(&program2[0..]);
    let mut blkfetch : LRU4x64<Instruction<u32,i32>, MemFetch<Instruction<u32,i32>>> = mem::LRU4x64::new(memfetch);
    println!("{:?}",state.eval_instrs(10_000_000, &mut blkfetch));
    // println!("{:?}", core::mem::size_of::<MathInstr<u32>>());

    let mut buf : Vec<u8> = Vec::new();
    for instr in PROGRAM {
        instr.write(&mut buf)
    }
    // println!("{:?}", buf);

    let mut read_instrs : [evaluator::Instruction<u32,i32>; 8] = [Halt; 8];
    let res = prim::read_buf(read_instrs.iter_mut(), &mut buf.iter().map(|x| *x));
    // println!("{:?}", res);
    // println!("{:?}", read_instrs);
}

impl evaluator::WriteSink for Vec<u8> {
    fn write(&mut self, val : u8) -> () {
        self.push(val);
    }
}