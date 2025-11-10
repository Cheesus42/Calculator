// 1+2+3 = 12+3+ -> 33+ -> 6
// (1+2)*3 = 12+3*
// 4*5+(2+3) = 23+45*+
//

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
}
fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}
enum Assoc {
    Left,
    Right,
    Invalid,
}
fn associativity(op: char) -> Assoc {
    match op {
        '+' | '-' | '*' | '/' => Assoc::Left,
        '^' => Assoc::Right,
        _ => Assoc::Invalid,
    }
}
fn main() {
    let mut expression = ExpStack::new();
    let exp = String::from("(  1+2)*3");
    expression.expression_to_stack(&exp);
    expression.shunting_yard();
    println!("{:?}", expression.output_stack)
}
