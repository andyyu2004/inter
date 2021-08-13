use crate::{compile_expr, exec_expr, parse_expr};

#[test]
fn test_exec() {
    let value = exec_expr!("3 - 4 * 1").unwrap();
    assert_eq!(value.try_into_int().unwrap(), -1);
}
