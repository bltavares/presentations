/// Realiza um grande calculo que te deixará surpreso
/// # Examples
///
/// ```
/// use minha_lib::*;
///
/// assert_eq!(5, calculo_surpresa(1, 2));
/// ```
pub fn calculo_surpresa(x: i32, y: i32, z: i32) -> i32 {
    x + y + z
}

pub fn super_calculo(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_super_calculo() {
    assert_eq!(3, super_calculo(1, 2));
}
