// 1+2+3 = 12+3+ -> 33+ -> 6
// (1+2)*3 = 12+3*
// 4*5+(2+3) = 23+45*+
//

use std::error;

#[derive(Debug)]
struct ExpStack {
    push_stack: Vec<char>,
    operator_stack: Vec<char>,
    output_stack: Vec<char>,
}
impl ExpStack {
    // turning an expression in infix type into a vector of infix type with each element containing
    // an element
    fn new() -> Self {
        ExpStack {
            push_stack: Vec::new(),
            operator_stack: Vec::new(),
            output_stack: Vec::new(),
        }
    }
    fn expression_to_stack(&mut self, expression: &String) {
        for m in expression.chars() {
            if m == ' ' {
                continue;
            }
            if matches!(m, '+' | '-' | '*' | '/' | '(' | ')' | '[' | ']') {
                if m == '[' {
                    self.push_stack.push('(');
                } else if m == ']' {
                    self.push_stack.push(')');
                } else {
                    self.push_stack.push(m);
                }
                continue;
            }
            m.to_string()
                .parse::<u32>()
                .expect("Not a Valid expression. Allowed expressions: +, -, *, /, (, ), [, ]");
            self.push_stack.push(m);
        }
    }
    fn shunting_yard(&mut self) {
        for &e in &self.push_stack {
            // case operator
            if "+-*/".contains(e) {
                while let Some(&top) = self.operator_stack.last() {
                    if precedence(top) >= precedence(e) {
                        self.output_stack.push(self.operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                self.operator_stack.push(e);

            // case paranthesis
            } else if e == '(' {
                self.operator_stack.push(e);
            } else if e == ')' {
                while let Some(&top) = self.operator_stack.last() {
                    if top == '(' {
                        break;
                    }
                    self.output_stack.push(self.operator_stack.pop().unwrap());
                }
                self.operator_stack.pop();
            // case number
            } else {
                self.output_stack.push(e);
            }
        }
        // moving the remaining elements onto the output stack
        while let Some(element) = self.operator_stack.pop() {
            self.output_stack.push(element);
        }
    }
    fn solve_rpn(&mut self) -> u32 {
        let mut stack: Vec<u32> = Vec::new();
        while let Some(element) = self.output_stack.pop() {
            if element.is_ascii_digit() {
                stack.push(
                    element
                        .to_digit(10)
                        .expect("not a number or valid operator"),
                );
            } else {
                let a = stack.pop().expect("Failed to get number a");
                let b = stack.pop().expect("Failed to get number b");
                let result: u32 = match element {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b as u32,
                    _ => {
                        println!("Invalid operator");
                        0
                    }
                };
                stack.push(result);
            }
        }
        if stack.len() == 1 {
            return stack[0];
        } else {
            return 0;
        }
    }
}
fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn main() {
    let mut expression = ExpStack::new();
    let exp = String::from("3*(2+5 -9)/3*(4/5)");
    expression.expression_to_stack(&exp);
    expression.shunting_yard();
    println!("{:?}", expression.output_stack);
    let result = expression.solve_rpn();
    println!("{}", result)
}
