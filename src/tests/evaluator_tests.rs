use crate::evaluator::{evaluate, tokenize};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

fn assert_decimal_eq(result: Decimal, expected: f64) {
    let expected_decimal = Decimal::from_f64(expected).unwrap();
    assert!((result - expected_decimal).abs() < Decimal::new(1, 10));
}

#[test]
fn test_addition() {
    let tokens = tokenize("2 + 3").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 5.0);
}

#[test]
fn test_subtraction() {
    let tokens = tokenize("5 - 3").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 2.0);
}

#[test]
fn test_unary_minus() {
    let tokens = tokenize("-3").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, -3.0);
}

#[test]
fn test_multiplication() {
    let tokens = tokenize("4 * 3").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 12.0);
}

#[test]
fn test_division() {
    let tokens = tokenize("10 / 2").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 5.0);
}

#[test]
fn test_exponentiation() {
    let tokens = tokenize("2 ^ 3").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 8.0);
}

#[test]
fn test_negative_exponent() {
    let tokens = tokenize("2 ^ -2").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 0.25);
}

#[test]
fn test_operator_precedence() {
    let tokens = tokenize("2 + 3 * 4").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 14.0);
}

#[test]
fn test_parentheses() {
    let tokens = tokenize("(2 + 3) * 4").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 20.0);
}

#[test]
fn test_division_by_zero() {
    let tokens = tokenize("1 / 0").unwrap();
    let result = evaluate(&tokens);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("division by zero"));
}

#[test]
fn test_non_integer_exponent() {
    let tokens = tokenize("2 ^ 0.5").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 1.4142135623730951);
}

#[test]
fn test_mismatched_parentheses() {
    let tokens = tokenize("(2 + 3").unwrap();
    let result = evaluate(&tokens);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Mismatched parentheses")
    );
}

#[test]
fn test_empty_expression() {
    let result = tokenize("");
    assert!(result.is_ok());
    let tokens = result.unwrap();
    let eval_result = evaluate(&tokens);
    assert!(eval_result.is_err());
    assert!(
        eval_result
            .unwrap_err()
            .to_string()
            .contains("Invalid expression")
    );
}

#[test]
fn test_large_number_precision() {
    let tokens = tokenize("999999999999 * 999999999999").unwrap();
    let result = evaluate(&tokens).unwrap();
    let expected = Decimal::from_i128(999999999999i128).unwrap()
        * Decimal::from_i128(999999999999i128).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_modulo() {
    let tokens = tokenize("10 % 3").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 1.0);
}

#[test]
fn test_factorial() {
    let tokens = tokenize("5!").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 120.0);
}

#[test]
fn test_sqrt() {
    let tokens = tokenize("sqrt(16)").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 4.0);
}

#[test]
fn test_abs() {
    let tokens = tokenize("abs(-5)").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 5.0);
}

#[test]
fn test_complex_expression() {
    let tokens = tokenize("2 * (3 + 4) ^ 2 - sqrt(16)").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 94.0);
}

#[test]
fn test_negative_sqrt() {
    let tokens = tokenize("sqrt(-1)").unwrap();
    let result = evaluate(&tokens);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Cannot compute square root of negative number")
    );
}

#[test]
fn test_negative_factorial() {
    let tokens = tokenize("(-5)!").unwrap();
    let result = evaluate(&tokens);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Cannot compute factorial of negative number")
    );
}

#[test]
fn test_decimal_modulo() {
    let tokens = tokenize("10.5 % 3").unwrap();
    let result = evaluate(&tokens).unwrap();
    assert_decimal_eq(result, 1.5);
}
