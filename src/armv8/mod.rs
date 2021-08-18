#[derive(Clone, Copy)]
enum Armv8Reg {
    X0 = 0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

use Armv8Reg::X0;
use Armv8Reg::X1;
use Armv8Reg::X2;
use Armv8Reg::X3;
use Armv8Reg::X4;
use Armv8Reg::X5;
use Armv8Reg::X6;
use Armv8Reg::X7;
use Armv8Reg::X8;
use Armv8Reg::X9;
use Armv8Reg::X10;
use Armv8Reg::X11;
use Armv8Reg::X12;
use Armv8Reg::X13;
use Armv8Reg::X14;
use Armv8Reg::X15;
use Armv8Reg::X16;
use Armv8Reg::X17;
use Armv8Reg::X18;
use Armv8Reg::X19;
use Armv8Reg::X20;
use Armv8Reg::X21;
use Armv8Reg::X22;
use Armv8Reg::X23;
use Armv8Reg::X24;
use Armv8Reg::X25;
use Armv8Reg::X26;
use Armv8Reg::X27;
use Armv8Reg::X28;
use Armv8Reg::X29;
use Armv8Reg::X30;
use Armv8Reg::X31;

enum Armv8Instruction {
    Add { a: Armv8Reg, b: Armv8Reg, c: Armv8Reg },
    Ret,
}

use Armv8Instruction::Add;
use Armv8Instruction::Ret;

fn emit_armv8_binary(code: Vec<Armv8Instruction>) -> Vec<u32> {
    code.iter().map(|instr| {
        match instr {
            Add { a, b, c } => 0x8B << 24 | emit_register_assignment(*a, *b, *c),
            Ret => 0xD65F03C0,
            _ => unimplemented!(),
        }
    }).collect()
}

fn emit_register_assignment(a: Armv8Reg, b: Armv8Reg, c: Armv8Reg) -> u32 {
    let c = (c as u32) << 16;
    let b = (b as u32) << 5;
    let a = a as u32;
    a | b | c
}

pub fn very_simple_add() -> Vec<u32> {
    let ret = emit_armv8_binary(vec![
        Armv8Instruction::Add { a: X0, b: X0, c: X0 },
        Armv8Instruction::Ret,
    ]);
    ret
}