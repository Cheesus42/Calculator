// 1+2+3 = 12+3+ -> 33+ -> 6
// (1+2)*3 = 12+3*
// 4*5+(2+3) = 23+45*+
//
//
pub mod calculator {

    #[derive(Debug, Clone)]
    enum Opts {
        Plus,
        Minus,
        Divide,
        Multiply,
        Modulo,
        Leftpar,
        Rightpar,
        Num(u32),
    }
    impl Opts {
        fn get_num(self) -> Result<u32, String> {
            match self {
                Opts::Num(n) => Ok(n),
                _ => Err("Expected Num variant".into()),
            }
        }
        fn precedence(&self) -> Result<u8, String> {
            match self {
                Opts::Plus | Opts::Minus => Ok(1),
                Opts::Multiply | Opts::Divide | Opts::Modulo => Ok(2),
                _ => Err("Invalid Operator".into()),
            }
        }
    }
    // turning an expression in infix type into a vector of infix type with each element containing
    // an element
    fn expression_to_stack(expression: &str, verbosity: u8) -> Result<Vec<Opts>, String> {
        let mut token_stack: Vec<Opts> = Vec::new();
        let mut big: bool = false;
        for m in expression.chars() {
            if m.is_ascii_digit() {
                let num = if big {
                    let a = token_stack.pop().ok_or("Bad Number Parse")?.get_num()?;
                    let b = m.to_digit(10).ok_or("Failed to convert to bigger Number")?;
                    a * 10 + b
                } else {
                    m.to_digit(10).ok_or("Invalid Number")?
                };
                token_stack.push(Opts::Num(num));
                big = true;
            } else {
                match m {
                    '+' => token_stack.push(Opts::Plus),
                    '-' => token_stack.push(Opts::Minus),
                    '*' => token_stack.push(Opts::Multiply),
                    '/' => token_stack.push(Opts::Divide),
                    '%' => token_stack.push(Opts::Modulo),
                    '(' | '[' => token_stack.push(Opts::Leftpar),
                    ')' | ']' => token_stack.push(Opts::Rightpar),
                    ' ' => continue,
                    _ => return Err(format!("Invalid Character {}", m)),
                }
                big = false;
            }
            if verbosity >= 4 {
                println!("Expression stack: {:?}", token_stack);
            }
        }
        Ok(token_stack)
    }
    fn shunting_yard(mut token_stack: Vec<Opts>, verbosity: u8) -> Result<Vec<Opts>, String> {
        let mut operator_stack: Vec<Opts> = Vec::new();
        let mut output_stack: Vec<Opts> = Vec::new();
        for element in token_stack.drain(..) {
            // case operator

            if !matches!(element, Opts::Leftpar | Opts::Rightpar | Opts::Num(_)) {
                // while precedence of the current token (element) is smaller or equal to the operator on
                // the operator stack, it pushes the headof the operator stack to the output
                while let Some(top) = operator_stack.last() {
                    if matches!(top, Opts::Leftpar) {
                        break;
                    }
                    if element.precedence() <= operator_stack.last().unwrap().precedence() {
                        output_stack.push(
                            operator_stack
                                .pop()
                                .expect("Unexpected empty operator stack"),
                        );
                    } else {
                        break;
                    }
                }
                operator_stack.push(element);

            // case paranthesis
            } else if matches!(element, Opts::Leftpar) {
                operator_stack.push(element);
            } else if matches!(element, Opts::Rightpar) {
                // runs until it finds the matching paranthesis and pops the operators from the
                // stack
                while let Some(top) = operator_stack.last() {
                    if matches!(top, Opts::Leftpar) {
                        break;
                    }
                    output_stack.push(operator_stack.pop().unwrap());
                    println!("{:?}", operator_stack.last());
                }
                // if it never finds the matching paranthesis it returns an error
                if operator_stack.pop().is_none() {
                    return Err("Missmatched paranthesis".into());
                }
            // case number
            } else {
                output_stack.push(element);
            }
            if verbosity >= 1 {
                println!("shunting_yard output stack: {:?}", output_stack);
                if verbosity >= 2 {
                    println!("Operator Stack: {:?}", operator_stack)
                }
            }
        }
        // moving the remaining elements onto the output stack
        while let Some(element) = operator_stack.pop() {
            output_stack.push(element);
        }
        println!("Final RPN Stack: {:?}", output_stack);
        Ok(output_stack)
    }
    fn solve_rpn(mut token_stack: Vec<Opts>, verb: u8) -> Result<u32, String> {
        let mut number_stack: Vec<Opts> = Vec::new();
        for element in token_stack.drain(..) {
            if matches!(element, Opts::Num(_)) {
                number_stack.push(element);
            } else {
                let b = number_stack.pop().ok_or("Missing Operand b")?.get_num()?;
                let a = number_stack.pop().ok_or("Missing Operand a")?.get_num()?;
                let result: Opts = match element {
                    Opts::Plus => Opts::Num(a.saturating_add(b)),
                    Opts::Minus => Opts::Num(a.saturating_sub(b)),
                    Opts::Multiply => Opts::Num(a.saturating_mul(b)),
                    Opts::Divide => {
                        if b == 0 {
                            return Err("Division by Zero".into());
                        }
                        Opts::Num(a.saturating_div(b))
                    }
                    Opts::Modulo => {
                        if b == 0 {
                            return Err("Modulo Division by Zero".into());
                        }
                        Opts::Num(a % b)
                    }
                    _ => return Err("Invalid Operator".into()),
                };
                number_stack.push(result);
            }
            if verb >= 3 {
                println!("Stack: {:?}", number_stack)
            }
        }
        number_stack
            .pop()
            .ok_or("Failed to receive returning Operand from stack")?
            .get_num()
    }
    pub fn verbose_eval(expression: &str, verbosity: u8) -> Result<u32, String> {
        let expression = expression_to_stack(expression, verbosity)?;
        let tokens = shunting_yard(expression, verbosity)?;
        solve_rpn(tokens, verbosity)
    }
    pub fn eval(expression: &str) -> Result<u32, String> {
        verbose_eval(expression, 0)
    }
}
