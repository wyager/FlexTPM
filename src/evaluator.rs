
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Reg {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, RA, RB, RC, RD, RE, RF
}

pub trait Fin where Self : 'static + Sized + Copy {
    const ARR : &'static [Self];
}

pub trait Read {
    fn read<It : Iterator<Item=u8>>(&mut It) -> Result<Self,()>
        where Self : Sized;
}

pub trait WriteSink {
    fn write(&mut self, u8) -> ();
}

pub trait Write {
    fn write<Sink : WriteSink>(&self, sink: &mut Sink) -> ();
}

impl<T : Fin> Read for T {
    fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<T,()> {
        match it.next() {
            Some(n) => match T::ARR.get(n as usize) {
                Some(t) => Ok(*t),
                None => Err(())
            },
            None => Err(())
        }  
    }
} 

use self::Reg::*;
impl Fin for Reg {
    const ARR : &'static [Reg] = &[R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,RA,RB,RC,RD,RE,RF];
}

impl<T : Fin + Eq> Write for T where {
    fn write<Sink:WriteSink>(&self,sink:&mut Sink) {
        match T::ARR.iter().position(|x| x == self) {
            None => panic!("Element is not present in its finite array."),
            Some(pos) => sink.write(pos as u8)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum M2Op {
    Add, Sub, Mul, Div, Equ, Lt, Gt
}

impl Fin for M2Op {
    const ARR : &'static [M2Op] = 
        &[M2Op::Add,M2Op::Sub,M2Op::Mul,
          M2Op::Div,M2Op::Equ,M2Op::Lt,M2Op::Gt];
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dir {
    Read, Write
}

impl Fin for Dir {
    const ARR : &'static [Dir] = &[Dir::Read, Dir::Write];
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Sign {
    Pos, Neg
}

impl Fin for Sign {
    const ARR : &'static [Sign] = &[Sign::Pos, Sign::Neg];
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cond {
    Always, EqZ, GtZ, LtZ
}

impl Fin for Cond {
    const ARR : &'static [Cond] = &[Cond::Always, Cond::EqZ, Cond::GtZ, Cond::LtZ];
}


// impl<W:Read> Read for MathInstr<W> {
//     fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<MathInstr<W>,()> {
//         match it.next() {
//             Some(n) => match n {
                
//                 _ => Err(())
//             },
//             None => Err(())
//         }  
//     }
// }

// impl<W : Prim + Write> Write for MathInstr<W> {
//     fn write<Sink : WriteSink>(&self, sink: &mut Sink) -> () {
//         match *self {
//             MathInstr::Lit{val,reg} => {sink.write(0x0); val.write(sink); reg.write(sink);},
//             MathInstr::Math2{op,r1,r2,r3} => {sink.write(0x1);op.write(sink);r1.write(sink);r2.write(sink);r3.write(sink)},
//             MathInstr::Jump{cond,flag,dest} => {sink.write(0x2);cond.write(sink);flag.write(sink);dest.write(sink)}
//         }
//     }
// }


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction<U> {
    Lit {val : U, reg : Reg},
    Um2 {op : M2Op, r1 : Reg, r2 : Reg, r3 : Reg},
    Sm2 {op : M2Op, r1 : Reg, r2 : Reg, r3 : Reg},
    UJump {cond : Cond, flag : Reg, dest : Reg},
    SJump {cond : Cond, flag : Reg, dest : Reg},
    Ram {dir : Dir, ptr : Reg, val : Reg},
    Out {reg : Reg},
    Call {major : Reg, minor : Reg, arg : Reg, len : Reg},
    Halt,
    Invalid // If the evaluator ever hits one of these, it means the person who wrote the instruction fetcher fucked up
}

impl<U> Default for Instruction<U> {
    fn default() -> Instruction<U> {
        Instruction::Invalid
    }
}

impl<U:Read> Read for Instruction<U> {
    fn read<It:Iterator<Item=u8>>(it:&mut It) -> Result<Instruction<U>,()> {
        match it.next() {
            Some(n) => match n {
                0x0 => 
                U::read(it).and_then(
                |val| Reg::read(it).map(
                |reg| Instruction::Lit{val, reg})),
                0x1 => 
                M2Op::read(it).and_then(
                |op| Reg::read(it).and_then(
                |r1| Reg::read(it).and_then(
                |r2| Reg::read(it).map(
                |r3| Instruction::Um2{op,r1,r2,r3})))),
                0x2 => 
                M2Op::read(it).and_then(
                |op| Reg::read(it).and_then(
                |r1| Reg::read(it).and_then(
                |r2| Reg::read(it).map(
                |r3| Instruction::Sm2{op,r1,r2,r3})))),
                0x3 => 
                Cond::read(it).and_then(
                |cond| Reg::read(it).and_then(
                |flag| Reg::read(it).map(
                |dest| Instruction::UJump{cond,flag,dest}))),
                0x4 => 
                Cond::read(it).and_then(
                |cond| Reg::read(it).and_then(
                |flag| Reg::read(it).map(
                |dest| Instruction::SJump{cond,flag,dest}))),
                0x5 => 
                Dir::read(it).and_then(|dir|
                Reg::read(it).and_then(|ptr|
                Reg::read(it).and_then(|val|
                Ok(Instruction::Ram{dir,ptr,val})))),
                0x6 => 
                Reg::read(it).and_then(|reg|
                Ok(Instruction::Out{reg})),
                0x7 => 
                Reg::read(it).and_then(|major|
                Reg::read(it).and_then(|minor|
                Reg::read(it).and_then(|arg|
                Reg::read(it).and_then(|len|
                Ok(Instruction::Call{major,minor,arg,len}))))),
                0x8 => Ok(Instruction::Halt),
                0x9 => Ok(Instruction::Invalid),
                _ => Err(())
            },
            None => Err(())
        }  
    }
}


impl<U : Prim + Write> Write for Instruction<U> {
    fn write<Sink : WriteSink>(&self, sink: &mut Sink) -> () {
        match *self {
            Instruction::Lit{val,reg} => {sink.write(0x0); val.write(sink); reg.write(sink);},
            Instruction::Um2{op,r1,r2,r3} => {sink.write(0x1);op.write(sink);r1.write(sink);r2.write(sink);r3.write(sink)},
            Instruction::Sm2{op,r1,r2,r3} => {sink.write(0x2);op.write(sink);r1.write(sink);r2.write(sink);r3.write(sink)},
            Instruction::UJump{cond,flag,dest} => {sink.write(0x3);cond.write(sink);flag.write(sink);dest.write(sink)}
            Instruction::SJump{cond,flag,dest} => {sink.write(0x4);cond.write(sink);flag.write(sink);dest.write(sink)}
            Instruction::Ram{dir,ptr,val} => {sink.write(0x5); dir.write(sink); ptr.write(sink); val.write(sink)},
            Instruction::Out{reg} => {sink.write(0x6); reg.write(sink)},
            Instruction::Call{major,minor,arg,len} => {sink.write(0x7);major.write(sink);minor.write(sink);arg.write(sink);len.write(sink)},
            Instruction::Halt => sink.write(0x8),
            Instruction::Invalid => sink.write(0x9)
        }
    }
}


use core::marker::PhantomData;

pub struct State<'a, U : 'a, S : 'a>  {
    pc : U,
    regs : [U;16],
    ram : &'a mut [U],
    _phantom : PhantomData<S>
}

fn reg2index(reg : Reg) -> usize {
    match reg {
        Reg::R0 => 0x0,
        Reg::R1 => 0x1,
        Reg::R2 => 0x2,
        Reg::R3 => 0x3,
        Reg::R4 => 0x4,
        Reg::R5 => 0x5,
        Reg::R6 => 0x6,
        Reg::R7 => 0x7,
        Reg::R8 => 0x8,
        Reg::R9 => 0x9,
        Reg::RA => 0xA,
        Reg::RB => 0xB,
        Reg::RC => 0xC,
        Reg::RD => 0xD,
        Reg::RE => 0xE,
        Reg::RF => 0xF
    }
}

#[derive(Debug)]
pub enum StaticNotice<U : Copy> {
    Call{major:U, minor:U, arg:U, len:U},
    Halt,
    Out{out:U}
}

#[derive(Debug)]
pub enum MutNotice<'a, U : 'a + Copy> {
    Thrash,
    Call{major:U, minor:U, slice:&'a mut [U]},
    Halt,
    Out{out:U}
}

#[derive(Copy, Clone, Debug)]
pub enum Failure<U : Copy> {
    CallOverflow,
    CallUnderflow,
    CodeOob {pc : U},
    RamOob {pc : U, addr : U, dir : Dir},
    InvalidInstruction
}

use core::ops::*;


pub trait Prim : Copy + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Ord + Shr<Self,Output=Self> + Shl<Self,Output=Self> + Shr<u8,Output=Self> + Shl<u8,Output=Self>
     {
    fn to_usize(&self) -> usize;
    fn zero() -> Self;
    fn one() -> Self;
}


pub trait Compl<O> : Prim where O:Compl<Self> {
    fn compl(&self) -> O;
}


impl<'a, U: 'a + Compl<S>, S: 'a + Compl<U>> State<'a,U,S> {

    fn set_reg(&mut self, reg : Reg, val : U) -> () {
        self.regs[reg2index(reg)] = val
    }

    fn get_reg(&mut self, reg : Reg) -> U {
        self.regs[reg2index(reg)]
    }

    fn set_ram(&mut self, ptr : U, val : U) -> () {
        self.ram[ptr.to_usize()] = val
    }

    fn get_ram(&mut self, ptr : U) -> U {
        self.ram[ptr.to_usize()]
    }

    fn eval_instr(&mut self, instr: &Instruction<U>) -> Option<Result<StaticNotice<U>, Failure<U>>> {
        use self::Instruction::*;
        // println!("{:?}", instr);
        let res = match *instr {
            Lit{val,reg} => {self.set_reg(reg, val); None},
            Um2{op,r1,r2,r3} => {
                let r1 = self.get_reg(r1);
                let r2 = self.get_reg(r2);
                let val = match op {
                    M2Op::Add => r1 + r2,
                    M2Op::Sub => r1 - r2,
                    M2Op::Mul => r1 * r2,
                    M2Op::Div => r1 / r2,
                    M2Op::Equ => if r1 == r2 {Prim::one()} else {Prim::zero()},
                    M2Op::Lt => if  r1 <  r2 {Prim::one()} else {Prim::zero()},
                    M2Op::Gt => if  r1 >  r2 {Prim::one()} else {Prim::zero()}
                };
                self.set_reg(r3,val);
                None
            },
            Sm2{op,r1,r2,r3} => {
                let r1 = self.get_reg(r1).compl();
                let r2 = self.get_reg(r2).compl();
                let val = match op {
                    M2Op::Add => r1 + r2,
                    M2Op::Sub => r1 - r2,
                    M2Op::Mul => r1 * r2,
                    M2Op::Div => r1 / r2,
                    M2Op::Equ => if r1 == r2 {Prim::one()} else {Prim::zero()},
                    M2Op::Lt =>  if r1 <  r2 {Prim::one()} else {Prim::zero()},
                    M2Op::Gt =>  if r1 >  r2 {Prim::one()} else {Prim::zero()}
                };
                self.set_reg(r3,val.compl());
                None
            },
            UJump{..} => None,
            SJump{..} => None,
            Out{reg} => Some (Ok (StaticNotice::Out{out:(self.get_reg(reg))})),
            Ram{dir,ptr,val} => {
                let ptr = self.get_reg(ptr);
                if ptr.to_usize() > self.ram.len() {
                    Some (Err (Failure::RamOob {pc:self.pc, addr:ptr, dir:dir}))
                }
                else {
                    match dir{
                        Dir::Read => {
                            let word = self.get_ram(ptr);
                            self.set_reg(val,word);
                        },
                        Dir::Write => {
                            let word = self.get_reg(val);
                            self.set_ram(ptr,word)
                        }
                    };
                    None
                }
            },
            Instruction::Call{major,minor,arg,len} => Some ({
                Ok(StaticNotice::Call{
                    major:self.get_reg(major),
                    minor:self.get_reg(minor),
                    arg:self.get_reg(arg),
                    len:self.get_reg(len)})
            }),
            Halt => Some(Ok(StaticNotice::Halt)),
            Invalid => Some(Err(Failure::InvalidInstruction))
        };
        let pc = match *instr {
            UJump{cond, flag, dest} => {
                let jump = match cond {
                    Cond::Always => true,
                    Cond::EqZ => self.get_reg(flag) == Prim::zero(),
                    Cond::GtZ => self.get_reg(flag) >  Prim::zero(),
                    Cond::LtZ => false
                };
                if jump { self.get_reg(dest) } else { self.pc + Prim::one() }
            },
            SJump{cond, flag, dest} => {
                let jump = match cond {
                    Cond::Always => true,
                    Cond::EqZ => self.get_reg(flag).compl() == Prim::zero(),
                    Cond::GtZ => self.get_reg(flag).compl() >  Prim::zero(),
                    Cond::LtZ => self.get_reg(flag).compl() <  Prim::zero(),
                };
                if jump { self.get_reg(dest) } else { self.pc + Prim::one() }
            },
            _ => self.pc + Prim::one()
        };
        self.pc = pc;
        res
    }

    pub fn eval_instrs<'t, F:Fetcher<U,Instruction<U>>>(&'t mut self, thrash_cnt : U, instrs : &mut F) -> Result<MutNotice<'t, U>, Failure<U>>  {
        for _ in 0..thrash_cnt.to_usize() {
            let instr = match instrs.fetch(self.pc) {
                None => return Err(Failure::CodeOob{pc:self.pc}),
                Some(instr) => instr
            };
            // if(self.pc.to_usize() >= instrs.len()) {};
            // let instr = &instrs[self.pc.to_usize()];
            let res = self.eval_instr(&instr);
            if let Some(interruption) = res {
                return match interruption {
                    Ok(notice) => match notice {
                        StaticNotice::Call {major, minor, arg, len} => {
                            self.get_call_slice(arg,len)
                                .map(|slice| MutNotice::Call{major,minor,slice})
                        },
                        StaticNotice::Halt => Ok(MutNotice::Halt),
                        StaticNotice::Out{out} => Ok(MutNotice::Out{out})
                    },
                    Err(err) => Err(err)
                }
               
            }
        }
        Ok(MutNotice::Thrash)
    }


  pub fn new(ram : &'a mut [U]) -> Self {
    State{pc:Prim::zero(), regs:[Prim::zero();16], ram:ram, _phantom:PhantomData::default()}
  }

  fn get_call_slice(&mut self, arg : U, len : U) -> Result<&mut [U], Failure<U>> {
    let end = arg + len;
    if end < arg {return Err(Failure::CallUnderflow)};
    if end.to_usize() >= self.ram.len() {return Err(Failure::CallOverflow)};
    let slice = &mut self.ram[arg.to_usize() .. end.to_usize()];
    Ok(slice)
  }

}


pub trait Fetcher<Ptr, T> {
    fn fetch(&mut self, ptr : Ptr) -> Option<T>;
}


// #[derive(Copy, Clone, Debug)]
// struct Lib_id {}

// #[derive(Copy, Clone, Debug)]
// struct Prog_id {}

// #[derive(Copy, Clone, Debug)]
// struct UUID<L> {
//     a : u64,
//     b : u64,
//     _l : PhantomData<L>
// }

// struct Library<'a,U : 'a> {
//     name : &'a str,
//     description : &'a str,
//     uuid : UUID<Lib_id>,
//     functions : &'a [fn(&mut [U]) -> Result<(),&'a str>]
// }


// struct Program<'a, U : 'a, S : 'a> {
//     ram_required : usize,
//     instr_count : usize,
//     uuid : UUID<Prog_id>,
//     instructions : &'a [Instruction<U,S>],
//     libraries    : &'a [Library<'a,U>]
// }


// struct Program_serialized<'a> {
//     ram_required : usize,
//     instr_count : usize,
//     instructions : &'a [u8],
//     libraries : &'a [UUID<Lib_id>]
// }

// pub trait Fetcher<Ptr, T> {
//     fn fetch(&mut self, ptr : Ptr) -> Option<T>;
// }

