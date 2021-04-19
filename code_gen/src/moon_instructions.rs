//! Define the MOON codes that can be emitted

use crate::register::Register;

pub const LABEL_WIDTH: usize = 24;

// This is just the max length of the below instructions
pub const INSTRUCTION_WIDTH: usize = 5;

const LOAD_W: &str = "lw";
const LOAD_B: &str = "lb";
const STORE_W: &str = "sw";
const STORE_B: &str = "sb";

const ADD: &str = "add";
const SUB: &str = "sub";
const MUL: &str = "mul";
const DIV: &str = "div";
const MOD: &str = "mod";
const AND: &str = "and";
const OR: &str = "or";
const NOT: &str = "not";

const CMP_EQ: &str = "ceq";
const CMP_NEQ: &str = "cne";
const CMP_LT: &str = "clt";
const CMP_LTE: &str = "cle";
const CMP_GT: &str = "cgt";
const CMP_GTE: &str = "cge";

const ADD_I: &str = "addi";
const SUB_I: &str = "subi";
const MUL_I: &str = "muli";
const DIV_I: &str = "divi";
const MOD_I: &str = "modi";
const AND_I: &str = "andi";
const OR_I: &str = "ori";

const CMP_EQ_I: &str = "ceqi";
const CMP_NEQ_I: &str = "cnei";
const CMP_LT_I: &str = "clti";
const CMP_LTE_I: &str = "clei";
const CMP_GT_I: &str = "cgei";
const CMP_GTE_I: &str = "cge";

const LSHIFT: &str = "sl";
const RSHIFT: &str = "sr";

const GETC: &str = "getc";
const PUTC: &str = "putc";

const JMP_ZERO: &str = "bz";
const JMP_NZERO: &str = "bnz";
const JMP: &str = "j";
const JMP_REG: &str = "jr";
const JMP_LNK: &str = "jl";
const JMP_LNK_REG: &str = "jlr";

const NOOP: &str = "nop";
const HALT: &str = "hlt";

const ENTRY: &str = "entry";
const ALIGN: &str = "align";
const ORG: &str = "org";
const MEM_STORE_W: &str = "dw"; // Like a store-immediate. This leaves the data addressable at the address this directive is executed
const MEM_STORE_B: &str = "db"; // Sme as above
const RES: &str = "res";

pub fn labeled_line(label: &str, instruction: &str) -> String {
    format!(" {:w$} {}", label, instruction, w = LABEL_WIDTH)
}

pub fn instr_line(instruction: &str) -> String {
    format!(" {:w$} {}", "", instruction, w = LABEL_WIDTH)
}

pub fn cmt_line(comment: &str) -> String {
    format!(" {:w$} {}", "", comment, w = LABEL_WIDTH)
}

// For convenience a function is created for each that simply copies the string out
// load_w()
pub fn load_w(ri: &Register, k: &str, rj: &Register) -> String {
    format!("{:w$} {},{}({})", LOAD_W, ri, k, rj, w = INSTRUCTION_WIDTH)
}
pub fn load_b(ri: &Register, k: &str, rj: &Register) -> String {
    format!("{:w$} {},{}({})", LOAD_B, ri, k, rj, w = INSTRUCTION_WIDTH)
}
pub fn store_w(k: &str, rj: &Register, ri: &Register) -> String {
    format!("{:w$} {}({}),{}", STORE_W, k, rj, ri, w = INSTRUCTION_WIDTH)
}
pub fn store_b(k: &str, rj: &Register, ri: &Register) -> String {
    format!("{:w$} {}({}),{}", STORE_B, k, rj, ri, w = INSTRUCTION_WIDTH)
}

pub fn add(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(ADD, dest, lhs, rhs)
}
pub fn sub(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(SUB, dest, lhs, rhs)
}
pub fn mul(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(MUL, dest, lhs, rhs)
}
pub fn div(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(DIV, dest, lhs, rhs)
}
pub fn _mod(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    // Note: Underscore
    format_instr_triple(MOD, dest, lhs, rhs)
}
pub fn and(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(AND, dest, lhs, rhs)
}
pub fn or(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(OR, dest, lhs, rhs)
}
pub fn not() -> String {
    NOT.to_string()
}

pub fn cmp_eq(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(CMP_EQ, dest, lhs, rhs)
}
pub fn cmp_neq(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(CMP_NEQ, dest, lhs, rhs)
}
pub fn cmp_lt(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(CMP_LT, dest, lhs, rhs)
}
pub fn cmp_lte(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(CMP_LTE, dest, lhs, rhs)
}
pub fn cmp_gt(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(CMP_GT, dest, lhs, rhs)
}
pub fn cmp_gte(dest: &Register, lhs: &Register, rhs: &Register) -> String {
    format_instr_triple(CMP_GTE, dest, lhs, rhs)
}

pub fn add_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(ADD_I, dest, lhs, rhs)
}
pub fn sub_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(SUB_I, dest, lhs, rhs)
}
pub fn mul_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(MUL_I, dest, lhs, rhs)
}
pub fn div_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(DIV_I, dest, lhs, rhs)
}
pub fn mod_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(MOD_I, dest, lhs, rhs)
}
pub fn and_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(AND_I, dest, lhs, rhs)
}
pub fn or_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(OR_I, dest, lhs, rhs)
}

pub fn cmp_eq_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(CMP_EQ_I, dest, lhs, rhs)
}
pub fn cmp_neq_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(CMP_NEQ_I, dest, lhs, rhs)
}
pub fn cmp_lt_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(CMP_LT_I, dest, lhs, rhs)
}
pub fn cmp_lte_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(CMP_LTE_I, dest, lhs, rhs)
}
pub fn cmp_gt_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(CMP_GT_I, dest, lhs, rhs)
}
pub fn cmp_gte_i(dest: &Register, lhs: &Register, rhs: &str) -> String {
    format_instr_triple_str(CMP_GTE_I, dest, lhs, rhs)
}

pub fn lshift(ri: &Register, k: &str) -> String {
    format_instr_double_str(LSHIFT, ri, k)
}
pub fn rshift(ri: &Register, k: &str) -> String {
    format_instr_double_str(RSHIFT, ri, k)
}

pub fn getc(ri: &Register) -> String {
    format_instr_single(GETC, ri)
}
pub fn putc(ri: &Register) -> String {
    format_instr_single(PUTC, ri)
}

pub fn jmp_zero(ri: &Register, k: &str) -> String {
    format_instr_double_str(JMP_ZERO, ri, k)
}
pub fn jmp_nzero(ri: &Register, k: &str) -> String {
    format_instr_double_str(JMP_NZERO, ri, k)
}
pub fn jmp(k: &str) -> String {
    format!("{:w$} {}", JMP, k, w = INSTRUCTION_WIDTH)
}
pub fn jmp_reg(ri: &Register) -> String {
    format!("{:w$} {}", JMP_REG, ri, w = INSTRUCTION_WIDTH)
}
pub fn jmp_lnk(ri: &Register, k: &str) -> String {
    format_instr_double_str(JMP_LNK, ri, k)
}
pub fn jmp_lnk_reg(ri: &Register, rj: &Register) -> String {
    format_instr_double(JMP_LNK_REG, ri, rj)
}

pub fn noop() -> String {
    format!("{:w$}", NOOP, w = INSTRUCTION_WIDTH)
}

pub fn halt() -> String {
    format!("{:w$}", HALT, w = INSTRUCTION_WIDTH)
}

pub fn entry() -> String {
    format!("{:w$}", ENTRY, w = INSTRUCTION_WIDTH)
}
pub fn align() -> String {
    format!("{:w$}", ALIGN, w = INSTRUCTION_WIDTH)
}
pub fn org(k: &str) -> String {
    format_instr_single_str(ORG, k)
}
/// dw
pub fn mem_store_w(ks: &[&str]) -> String {
    let mut result = String::new();
    result.push_str(&format!("{:w$}", MEM_STORE_W, w = INSTRUCTION_WIDTH));

    if ks.len() == 0 {
        panic!("Store Words with empty arguments");
    }

    result.push_str(&format!("{}", ks[0]));

    for element in ks.iter().skip(1) {
        result.push_str(&format!(", {}", element));
    }

    result
}
/// db
pub fn mem_store_b(ks: &[&str]) -> String {
    let mut result = String::new();
    result.push_str(&format!("{:w$}", MEM_STORE_W, w = INSTRUCTION_WIDTH));

    if ks.len() == 0 {
        panic!("Store Words with empty arguments");
    }

    result.push_str(&format!("{}", ks[0]));

    for element in ks.iter().skip(1) {
        result.push_str(&format!(", {}", element));
    }

    result
}
pub fn res(k: &str) -> String {
    format_instr_single_str(RES, k)
}

fn format_instr_triple<T: std::fmt::Display>(instruction: &str, a: &T, b: &T, c: &T) -> String {
    format!(
        "{:w$} {}, {}, {}",
        instruction,
        a,
        b,
        c,
        w = INSTRUCTION_WIDTH
    )
}

fn format_instr_triple_str<T: std::fmt::Display>(
    instruction: &str,
    a: &T,
    b: &T,
    c: &str,
) -> String {
    format!(
        "{:w$} {}, {}, {}",
        instruction,
        a,
        b,
        c,
        w = INSTRUCTION_WIDTH
    )
}

fn format_instr_double<T: std::fmt::Display>(instruction: &str, a: &T, b: &T) -> String {
    format!("{:w$} {}, {}", instruction, a, b, w = INSTRUCTION_WIDTH)
}

fn format_instr_double_str<T: std::fmt::Display>(instruction: &str, a: &T, b: &str) -> String {
    format!("{:w$} {}, {}", instruction, a, b, w = INSTRUCTION_WIDTH)
}

fn format_instr_single<T: std::fmt::Display>(instruction: &str, a: &T) -> String {
    format!("{:w$} {}", instruction, a, w = INSTRUCTION_WIDTH)
}

fn format_instr_single_str(instruction: &str, a: &str) -> String {
    format!("{:w$} {}", instruction, a, w = INSTRUCTION_WIDTH)
}
