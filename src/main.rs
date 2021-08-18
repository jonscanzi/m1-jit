#![allow(unused_imports)]
#![allow(dead_code)]
use std::{thread, time};
use std::ptr::null_mut;
use std::os::raw::c_int;
use std::ffi::CString;
use libc::{MAP_JIT, self, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, c_void};
use std::mem::transmute;
mod armv8;
use armv8::very_simple_add;

fn main() {
    let binary_code = very_simple_add();
    unsafe {
        let addr: *mut c_void = null_mut();
        let len = 128;
        let mut prot = 0;
        prot |= PROT_EXEC;
        prot |= PROT_READ;
        prot |= PROT_WRITE;

        let flags = MAP_JIT | MAP_ANONYMOUS | MAP_PRIVATE;
        pthread_jit_write_protect_np(false);
        
        let fd = -1;
        let offset = 0;
        let mem: *mut u32 = libc::mmap(addr, len, prot, flags, fd, offset) as *mut u32;
        
        for (pos, c) in binary_code.iter().enumerate() {
            mem.offset(pos as isize).write(*c);
        }
        // mem.write(0x8B000000); // add x0, x0, #2
        // mem.offset(1).write(0xd65f03c0); // RET
        pthread_jit_write_protect_np(true);
        
        let mem = transmute::<*const u32, fn(u64) -> u64>(mem);
        thread::sleep(time::Duration::from_millis(1000));
        println!("{:?}", mem(10));
    }
}

unsafe fn print_raw_c_str(raw: *const u8) {
    let mut offset = 0;
    loop {
        let chr = raw.offset(offset).read();
        if chr == 0 {
            break
        }
        else {
            print!("{}", chr as char);
        }
        offset+=1;
    }
    println!();
}

#[link(name = "pthread")]
extern "C" {
    fn pthread_jit_write_protect_np(enabled: bool);
}

// #[link(name = "libkern")]
// extern "C" {
//     fn sys_icache_invalidate(start: *mut c_void, len: usize);
// }

// enum Amd64Reg {
//     Rax,
//     Rbx,
//     Rcx
// }

// enum Amd64Instruction {
//     Add(Amd64Reg, Amd64Reg),
//     Ret,
// }

// fn amd64_to_armv8(instruction: Amd64Instruction) -> Armv8Instruction {
//     match instruction {
//         Amd64Instruction::Add(a, b) => unimplemented!(),//Armv8Instruction::Add()
//         _ => unimplemented!(),
//     }
// }