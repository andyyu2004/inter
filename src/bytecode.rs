#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
    ipush(i64),
    pop,
    iadd,
    isub,
    imul,
    idiv,
}
