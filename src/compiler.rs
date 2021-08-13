use crate::ast::{BinOp, Expr};
use crate::bytecode::Opcode;
use crate::Result;

pub struct Compiler {
    pub(crate) code: Vec<Opcode>,
}

impl Compiler {
    pub fn new() -> Self {
        Self { code: Default::default() }
    }

    fn emit(&mut self, opcode: Opcode) {
        self.code.push(opcode);
    }

    pub fn compile_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            Expr::Bin(lhs, op, rhs) => {
                self.compile_expr(rhs);
                self.compile_expr(lhs);
                let opcode = match op {
                    BinOp::Plus => Opcode::iadd,
                    BinOp::Minus => Opcode::isub,
                    BinOp::Multiply => Opcode::imul,
                    BinOp::Divide => Opcode::idiv,
                };
                self.emit(opcode)
            }
            Expr::Unary(op, expr) => self.compile_expr(expr)?,
            Expr::Int(i) => self.emit(Opcode::ipush(*i)),
        }
        Ok(())
    }

    pub fn finish(self) -> Vec<Opcode> {
        self.code
    }
}

#[macro_export]
macro_rules! compile_expr {
    ($src:expr) => {{
        let expr = parse_expr!($src);
        let mut compiler = $crate::compiler::Compiler::new();
        compiler.compile_expr(&expr).unwrap();
        compiler.finish()
    }};
}
