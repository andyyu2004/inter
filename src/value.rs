#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Int(v)
    }
}

impl Value {
    pub fn try_into_int(self) -> Result<i64, anyhow::Error> {
        if let Self::Int(v) = self { Ok(v) } else { bail!("expected int value found `{:?}`", self) }
    }
}
