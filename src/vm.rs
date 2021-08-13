use crate::bytecode::Opcode;
use crate::value::Value;
use crate::Result;

pub struct VM {
    stack: Vec<Value>,
}

macro_rules! int_op {
    ($vm:expr, $op:tt) => {{
        let lhs = $vm.pop().try_into_int()?;
        let rhs = $vm.pop().try_into_int()?;
        $vm.push(lhs $op rhs)
    }};
}

impl VM {
    pub fn new() -> Self {
        Self { stack: Default::default() }
    }

    pub fn exec(&mut self, code: &[Opcode]) -> Result<Value> {
        for &opcode in code {
            self.exec_op(opcode)?;
        }
        Ok(self.stack.pop().unwrap())
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.stack.push(value.into())
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn exec_op(&mut self, opcode: Opcode) -> Result<()> {
        match opcode {
            Opcode::ipush(i) => self.push(i),
            Opcode::iadd => int_op!(self, +),
            Opcode::isub => int_op!(self, -),
            Opcode::imul => int_op!(self, *),
            Opcode::idiv => int_op!(self, /),
            Opcode::pop => {
                self.pop();
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! exec_expr {
    ($src:expr) => {{
        let code = compile_expr!($src);
        let mut vm = $crate::vm::VM::new();
        vm.exec(&code)
    }};
}
