use calculator::calculator::{eval, verbose_eval};
#[test]
fn test_basic_addition() {
    assert_eq!(eval(&"1+1").unwrap(), 2);
    assert_eq!(eval(&"0+5").unwrap(), 5);
}

#[test]
fn test_basic_subtraction() {
    assert_eq!(eval(&"5-3").unwrap(), 2);
    assert_eq!(eval(&"3-5").unwrap(), 0); // floored at 0
    assert_eq!(eval(&"0-1").unwrap(), 0); // floored at 0
}

#[test]
fn test_basic_multiplication() {
    assert_eq!(eval(&"4*2").unwrap(), 8);
    assert_eq!(eval(&"0*5").unwrap(), 0);
}

#[test]
fn test_basic_division() {
    assert_eq!(eval(&"8/2").unwrap(), 4);
    assert_eq!(eval(&"5/2").unwrap(), 2); // integer division
}

#[test]
fn test_modulus() {
    assert_eq!(eval(&"5%2").unwrap(), 1);
    assert_eq!(eval(&"4%2").unwrap(), 0);
}

#[test]
fn test_operator_precedence() {
    assert_eq!(eval(&"2+3*4").unwrap(), 14);
    assert_eq!(eval(&"2*3+4").unwrap(), 10);
    assert_eq!(eval(&"10-3*2").unwrap(), 4); // 10 - 6
}

#[test]
fn test_parentheses() {
    assert_eq!(eval(&"(2+3)*4").unwrap(), 20);
    assert_eq!(eval(&"2*(3+4)").unwrap(), 14);
    assert_eq!(eval(&"(5-10)+3").unwrap(), 3); // floored at 0
}

#[test]
fn test_nested_parentheses() {
    assert_eq!(eval(&"((2+3)*(1+1))").unwrap(), 10);
    assert_eq!(eval(&"(2+(3*(2+1)))").unwrap(), 11);
}

#[test]
fn test_spaces_in_expression() {
    assert_eq!(eval(&" 2 + 3 * 4 ").unwrap(), 14);
    assert_eq!(eval(&" ( 2 + 3 ) * 4 ").unwrap(), 20);
}

#[test]
fn test_verbose_eval() {
    let result = verbose_eval(&"2+3*4", 1).unwrap();
    assert_eq!(result, 14);

    let result_high_verbosity = verbose_eval(&"2+3*4", 3).unwrap();
    assert_eq!(result_high_verbosity, 14);
}

#[test]
fn test_invalid_input() {
    assert!(eval(&"2++2").is_err());
    assert!(eval(&"abc").is_err());
    assert!(eval(&"2*/3").is_err());
}

#[test]
fn test_division_by_zero() {
    assert!(eval(&"5/0").is_err());
    assert!(eval(&"10%0").is_err());
}

#[test]
fn test_complex_expressions() {
    assert_eq!(eval(&"5+2*(3+7)-4/2").unwrap(), 23);
    assert_eq!(eval(&"((10-3)*2)%4 + 1").unwrap(), 3);
    assert_eq!(eval(&"(8/2)*(3+1)-5").unwrap(), 11);
}

#[test]
fn test_subtraction_flooring() {
    assert_eq!(eval(&"1-2").unwrap(), 0);
    assert_eq!(eval(&"0-10").unwrap(), 0);
    assert_eq!(eval(&"(5-10)-1").unwrap(), 0);
}

#[test]
fn test_multiple_digit_numbers() {
    assert_eq!(eval(&"12+34").unwrap(), 46);
    assert_eq!(eval(&"100-50*2").unwrap(), 0);
    assert_eq!(eval(&"50/2+25").unwrap(), 50);
}

#[test]
fn test_leons_test_test() {
    assert_eq!(verbose_eval(&"2+3*4", 4).unwrap(), 14);
}
#[test]
fn test_leons_test() {
    assert_eq!(verbose_eval(&"2+(3*4)", 4).unwrap(), 14);
}
