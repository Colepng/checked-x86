#![allow(unexpected_cfgs)]

use flux_core::eq;
use flux_rs::{defs, refined_by, reflect, spec};

use crate::bitvec::{BV32, BV64};

mod bitvec;

fn main() {
    println!("Hello, world!");
}

#[refined_by(regs: Registers)]
pub struct X86_64 {
    #[field(Registers[regs])]
    registers: Registers,
}

#[refined_by(rax: int, rbx: int, rcx: int, rdx: int, rsi: int, rdi: int, rsp: int, rbp: int)]
pub struct Registers {
    #[field(u64[rax])]
    rax: u64,
    #[field(u64[rbx])]
    rbx: u64,
    #[field(u64[rcx])]
    rcx: u64,
    #[field(u64[rdx])]
    rdx: u64,
    #[field(u64[rsi])]
    rsi: u64,
    #[field(u64[rdi])]
    rdi: u64,
    #[field(u64[rsp])]
    rsp: u64,
    #[field(u64[rbp])]
    rbp: u64,
}

defs! {
    fn update_reg_64(regs: Registers, dest_reg: Register64, value: int) -> Registers {
        if dest_reg == Register64::Rax {
            Registers {
                rax: value,
                rbx: regs.rbx,
                rcx: regs.rcx,
                rdx: regs.rdx,
                rsi: regs.rsi,
                rdi: regs.rdi,
                rsp: regs.rsp,
                rbp: regs.rbp
            }
        } else if dest_reg == Register64::Rbx {
            Registers {
                rax: regs.rax,
                rbx: value,
                rcx: regs.rcx,
                rdx: regs.rdx,
                rsi: regs.rsi,
                rdi: regs.rdi,
                rsp: regs.rsp,
                rbp: regs.rbp
            }
        } else if dest_reg == Register64::Rcx {
            Registers {
                rax: regs.rax,
                rbx: regs.rbx,
                rcx: value,
                rdx: regs.rdx,
                rsi: regs.rsi,
                rdi: regs.rdi,
                rsp: regs.rsp,
                rbp: regs.rbp
            }
        } else if dest_reg == Register64::Rdx {
            Registers {
                rax: regs.rax,
                rbx: regs.rbx,
                rcx: regs.rcx,
                rdx: value,
                rsi: regs.rsi,
                rdi: regs.rdi,
                rsp: regs.rsp,
                rbp: regs.rbp
            }
        } else if dest_reg == Register64::Rsi {
            Registers {
                rax: regs.rax,
                rbx: regs.rbx,
                rcx: regs.rcx,
                rdx: regs.rdx,
                rsi: value,
                rdi: regs.rdi,
                rsp: regs.rsp,
                rbp: regs.rbp
            }
        } else if dest_reg == Register64::Rdi {
            Registers {
                rax: regs.rax,
                rbx: regs.rbx,
                rcx: regs.rcx,
                rdx: regs.rdx,
                rsi: regs.rsi,
                rdi: value,
                rsp: regs.rsp,
                rbp: regs.rbp
            }
        } else if dest_reg == Register64::Rsp {
            Registers {
                rax: regs.rax,
                rbx: regs.rbx,
                rcx: regs.rcx,
                rdx: regs.rdx,
                rsi: regs.rsi,
                rdi: regs.rdi,
                rsp: value,
                rbp: regs.rbp
            }
        } else {
            Registers {
                rax: regs.rax,
                rbx: regs.rbx,
                rcx: regs.rcx,
                rdx: regs.rdx,
                rsi: regs.rsi,
                rdi: regs.rdi,
                rsp: regs.rsp,
                rbp: value,
            }
        }
    }
}

impl Registers {
    #[spec(fn (&Self[@regs], Register64[@reg]) -> u64[read_64_spec(regs, reg)])]
    fn read_64(&self, reg: Register64) -> u64 {
        match reg {
            Register64::Rax => self.rax,
            Register64::Rbx => self.rbx,
            Register64::Rcx => self.rcx,
            Register64::Rdx => self.rdx,
            Register64::Rsi => self.rsi,
            Register64::Rdi => self.rdi,
            Register64::Rsp => self.rsp,
            Register64::Rbp => self.rbp,
        }
    }

    #[spec(fn (self: &strg Self[@regs], dest_reg: Register64, value: u64) ensures self: Self[update_reg_64(regs, dest_reg, value)])]
    fn update_reg(&mut self, dest_reg: Register64, value: u64) {
        match dest_reg {
            Register64::Rax => self.rax = value,
            Register64::Rbx => self.rbx = value,
            Register64::Rcx => self.rcx = value,
            Register64::Rdx => self.rdx = value,
            Register64::Rsi => self.rsi = value,
            Register64::Rdi => self.rdi = value,
            Register64::Rsp => self.rsp = value,
            Register64::Rbp => self.rbp = value,
        }
    }
}

#[repr(u8)]
#[reflect]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register64 {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
}

eq!(Register64);

#[repr(u8)]
#[reflect]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register32 {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Esp,
    Ebp,
}

eq!(Register32);

pub enum Register16 {
    Ax,
}

pub enum Register8 {
    Al,
}

defs! {
    fn read_64_spec(regs: Registers, reg: Register64) -> int {
        if reg == Register64::Rax {
            regs.rax
        } else if reg == Register64::Rbx {
            regs.rbx
        } else if reg == Register64::Rcx {
            regs.rcx
        } else if reg == Register64::Rdx {
            regs.rdx
        } else if reg == Register64::Rsi {
            regs.rsi
        } else if reg == Register64::Rdi {
            regs.rdi
        } else if reg == Register64::Rsp {
            regs.rsp
        } else {
            regs.rbp
        }
    }

    fn reg_32_to_reg_64(reg: Register32) -> Register64 {
        if reg == Register32::Eax {
            Register64::Rax
        } else if reg == Register32::Ebx {
            Register64::Rbx
        } else if reg == Register32::Ecx {
            Register64::Rcx
        } else if reg == Register32::Edx {
            Register64::Rdx
        } else if reg == Register32::Esi {
            Register64::Rsi
        } else if reg == Register32::Edi {
            Register64::Rdi
        } else if reg == Register32::Esp {
            Register64::Rsp
        } else {
            Register64::Rbp
        }
    }

    fn read_32_spec(regs: Registers, reg: Register32) -> int {
        truncate_to_32(read_64_spec(regs, reg_32_to_reg_64(reg)))
    }

    fn truncate_to_32(value: int) -> int {
        bv_bv64_to_int(bv_and(bv_int_to_bv64(value), 0xFFFF_FFFF))
    }

    fn truncate_to_16(value: int) -> int {
        bv_bv64_to_int(bv_and(bv_int_to_bv64(value), 0xFFFF))
    }

    fn truncate_to_8high(value: int) -> int {
        bv_bv64_to_int(bv_lshr(bv_and(bv_int_to_bv64(value), 0xFF00), 8))
    }

    fn truncate_to_8low(value: int) -> int {
        bv_bv64_to_int(bv_and(bv_int_to_bv64(value), 0xFF))
    }
}

#[spec(fn (u64[@val]) -> u32[truncate_to_32(val)])]
fn u64_to_u32(value: u64) -> u32 {
    <BV64 as Into<u64>>::into(BV64::new(value) & 0xFFFF_FFFF) as u32
}

#[spec(fn (u64[@val]) -> u16[truncate_to_16(val)])]
fn u64_to_u16(value: u64) -> u16 {
    <BV64 as Into<u64>>::into(BV64::new(value) & 0xFFFF) as u16
}

#[spec(fn (u64[@val]) -> u8[truncate_to_8high(val)])]
fn u64_to_u8high(value: u64) -> u8 {
    <BV64 as Into<u64>>::into((BV64::new(value) & 0xFF00) >> 8) as u8
}

#[spec(fn (u64[@val]) -> u8[truncate_to_8low(val)])]
fn u64_to_u8low(value: u64) -> u8 {
    <BV64 as Into<u64>>::into(BV64::new(value) & 0xFF) as u8
}

// #[trusted]
// #[spec(fn (u64[@val]) -> u32[truncate_to_32(val)])]
// fn u64_to_u16(value: u64) -> u32 {
//     value as u32
// }
//
// // #[trusted]
// #[spec(fn (u64[@val]) -> u32[truncate_to_32(val)])]
// fn u64_to_u8(value: u64) -> u32 {
//     value as u32
// }

impl X86_64 {
    #[spec(fn (&X86_64[@regs], Register64[@tag]) -> u64[read_64_spec(regs.regs, tag)])]
    fn read_64(&self, reg: Register64) -> u64 {
        match reg {
            Register64::Rax => self.registers.rax,
            Register64::Rbx => self.registers.rbx,
            Register64::Rcx => self.registers.rcx,
            Register64::Rdx => self.registers.rdx,
            Register64::Rsi => self.registers.rsi,
            Register64::Rdi => self.registers.rdi,
            Register64::Rsp => self.registers.rsp,
            Register64::Rbp => self.registers.rbp,
        }
    }

    #[spec(fn (&X86_64[@regs], Register32[@tag]) -> u32[read_32_spec(regs.regs, tag)])]
    fn read_32(&self, reg: Register32) -> u32 {
        match reg {
            Register32::Eax => u64_to_u32(self.registers.rax),
            Register32::Ebx => u64_to_u32(self.registers.rbx),
            Register32::Ecx => u64_to_u32(self.registers.rcx),
            Register32::Edx => u64_to_u32(self.registers.rdx),
            Register32::Esi => u64_to_u32(self.registers.rsi),
            Register32::Edi => u64_to_u32(self.registers.rdi),
            Register32::Esp => u64_to_u32(self.registers.rsp),
            Register32::Ebp => u64_to_u32(self.registers.rbp),
        }
    }

    // fn read_16(&mut self, reg: Register16) -> u16 {
    //     match reg {
    //         Register16::Ax => self.registers.rax as u16,
    //     }
    // }
    //
    // fn read_8(&mut self, reg: Register8) -> u8 {
    //     match reg {
    //         Register8::Al => self.registers.rax as u8,
    //     }
    // }

    // loads a 64 byte value from a register and loads it into a memory address
    fn mov_m64_r64(&mut self, src_reg: Register64) {
        let value = self.read_64(src_reg);
    }

    // loads a 64 byte value from a register into a register
    #[spec(fn (self: &strg X86_64[@x86], src_reg: Register64, dest_reg: Register64) ensures self:X86_64[update_reg_64(x86.regs, dest_reg, read_64_spec(x86.regs, src_reg))])]
    fn mov_r64_r64(&mut self, src_reg: Register64, dest_reg: Register64) {
        self.registers
            .update_reg(dest_reg, self.registers.read_64(src_reg));
    }
}
