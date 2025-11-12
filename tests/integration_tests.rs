#[cfg(test)]
mod tests {

    use calculator::calculator;
    //
    // ─── BASIC OPERATIONS ─────────────────────────────────────────────────────────
    //

    #[test]
    fn addition_works() {
        assert_eq!(calculator::eval("1+2").unwrap(), 3);
        assert_eq!(calculator::eval("10 + 25").unwrap(), 35);
    }

    #[test]
    fn subtraction_works() {
        assert_eq!(calculator::eval("10-4").unwrap(), 6);
        assert_eq!(calculator::eval("50 - 25 - 5").unwrap(), 20);
    }

    #[test]
    fn multiplication_works() {
        assert_eq!(calculator::eval("3*4").unwrap(), 12);
        assert_eq!(calculator::eval("2*3*4").unwrap(), 24);
    }

    #[test]
    fn division_works() {
        assert_eq!(calculator::eval("8/2").unwrap(), 4);
        assert_eq!(calculator::eval("9 / 3 / 3").unwrap(), 1);
    }

    #[test]
    fn modulo_works() {
        assert_eq!(calculator::eval("10 % 3").unwrap(), 1);
        assert_eq!(calculator::eval("12 % 5").unwrap(), 2);
    }

    //
    // ─── OPERATOR PRECEDENCE AND PARENTHESES ───────────────────────────────────────
    //

    #[test]
    fn precedence_without_parentheses() {
        assert_eq!(calculator::eval("2+3*4").unwrap(), 14);
        assert_eq!(calculator::eval("10-6/3").unwrap(), 8);
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(calculator::eval("(2+3)*4").unwrap(), 20);
        assert_eq!(calculator::eval("10/(5-3)").unwrap(), 5);
    }

    #[test]
    fn nested_parentheses() {
        assert_eq!(calculator::eval("2*(3+(4*5))").unwrap(), 46);
    }

    #[test]
    fn mixed_brackets_work_as_parentheses() {
        assert_eq!(calculator::eval("[2+(3*4)]").unwrap(), 14);
        assert_eq!(
            calculator::eval("{ (2+3) * [4] }").is_err(),
            true,
            "Curly braces should produce error if not supported"
        );
    }

    //
    // ─── MULTI-DIGIT NUMBERS AND SPACING ──────────────────────────────────────────
    //

    #[test]
    fn multi_digit_numbers() {
        assert_eq!(calculator::eval("12+34").unwrap(), 46);
        assert_eq!(calculator::eval("100/25").unwrap(), 4);
    }

    #[test]
    fn spaces_are_ignored() {
        assert_eq!(calculator::eval("  2 +   3 * 4 ").unwrap(), 14);
    }

    //
    // ─── ERROR HANDLING ───────────────────────────────────────────────────────────
    //

    #[test]
    fn invalid_character() {
        let result = calculator::eval("2 + a");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid character: a");
    }

    #[test]
    fn mismatched_parentheses() {
        let result = calculator::eval("(2+3");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mismatched parentheses");
    }

    #[test]
    fn missing_operands() {
        let result = calculator::eval("2+*3");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing operand a");

        let result = calculator::eval("2+");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing operand b");
    }

    #[test]
    fn division_by_zero() {
        let result = calculator::eval("10 / 0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn modulo_by_zero() {
        let result = calculator::eval("10 % 0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Modulo by zero");
    }

    #[test]
    fn empty_expression_fails() {
        let result = calculator::eval("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No result found");
    }
}
