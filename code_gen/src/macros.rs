//! Actions that are more intuitively composed of more than one instructions

use crate::moon_instructions as moon;
use crate::register::{Register, R15};
use output_manager::OutputConfig;

pub fn res(k: usize, label: &str, output: &mut OutputConfig) {
    output.add_data(&moon::labeled_line(&label, &moon::res(&k.to_string())));
    output.add_data(&moon::instr_line(&moon::align()));
}

pub fn zero(r: &Register, output: &mut OutputConfig) {
    output.add_exec(&moon::sub(r, r, r));
}

pub fn cmt_exec(msg: &str, output: &mut OutputConfig) {
    output.add_exec(&moon::cmt_line(msg));
}

pub fn ret(output: &mut OutputConfig) {
    output.add_exec(&moon::instr_line(&moon::jmp_reg(&R15)));
}
