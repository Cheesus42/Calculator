// 1+2+3 = 12+3+ -> 33+ -> 6
// (1+2)*3 = 12+3*
// 4*5+(2+3) = 23+45*+
//
#[derive(Debug)]
enum Opts {
    Plus,
    Minus,
    Divide,
    Multiply,
    Leftpar,
    Rightpar,
    Num(u32),
}
impl Opts {
    fn get_num(self) -> u32 {
        match self {
            Opts::Num(n) => n,
            _ => panic!("Expected Num variant"),
        }
    }
}

#[derive(Debug)]
struct ExpStack {
    stack: Vec<Opts>,
}
impl ExpStack {
    // turning an expression in infix type into a vector of infix type with each element containing
    // an element
    fn new() -> Self {
        ExpStack { stack: Vec::new() }
    }
    fn expression_to_stack(&mut self, expression: &String) {
        let mut big: bool = false;
        for m in expression.chars() {
            if m.is_ascii_digit() {
                let num = if big {
                    let a = self
                        .stack
                        .pop()
                        .expect("Failed to fetch number probably because of false set big flag")
                        .get_num();
                    let b = m
                        .to_digit(10)
                        .expect("Failed to convert digit for bigger number");
                    a * 10 + b
                } else {
                    m.to_digit(10)
                        .expect("Failed to convert to number not in big")
                };
                self.stack.push(Opts::Num(num));
                big = true;
            } else {
                match m {
                    '+' => self.stack.push(Opts::Plus),
                    '-' => self.stack.push(Opts::Minus),
                    '*' => self.stack.push(Opts::Multiply),
                    '/' => self.stack.push(Opts::Divide),
                    '(' => self.stack.push(Opts::Leftpar),
                    ')' => self.stack.push(Opts::Rightpar),
                    '[' => self.stack.push(Opts::Leftpar),
                    ']' => self.stack.push(Opts::Rightpar),
                    _ => println!("Not a valid Operator. Valid Operators: +, -, *, /, (, ), [, ]"),
                }
            }
        }
    }
    // fn shunting_yard(&mut self) {
    //     for &e in &self.push_stack {
    //         // case operator
    //         if "+-*/".contains(e) {
    //             while let Some(&top) = self.operator_stack.last() {
    //                 if precedence(top) >= precedence(e) {
    //                     self.output_stack.push(self.operator_stack.pop().unwrap());
    //                 } else {
    //                     break;
    //                 }
    //             }
    //             self.operator_stack.push(e);

    //         // case paranthesis
    //         } else if e == '(' {
    //             self.operator_stack.push(e);
    //         } else if e == ')' {
    //             while let Some(&top) = self.operator_stack.last() {
    //                 if top == '(' {
    //                     break;
    //                 }
    //                 self.output_stack.push(self.operator_stack.pop().unwrap());
    //             }
    //             self.operator_stack.pop();
    //         // case number
    //         } else {
    //             self.output_stack.push(e);
    //         }
    //     }
    //     // moving the remaining elements onto the output stack
    //     while let Some(element) = self.operator_stack.pop() {
    //         self.output_stack.push(element);
    //     }
    // }
    // fn solve_rpn(&mut self) -> u32 {
    //     let mut stack: Vec<u32> = Vec::new();
    //     let mut input: Vec<char> = self.output_stack.clone();
    //     input.reverse();
    //     while let Some(element) = input.pop() {
    //         if element.is_ascii_digit() {
    //             stack.push(
    //                 element
    //                     .to_digit(10)
    //                     .expect("not a number or valid operator"),
    //             );
    //         } else if "+-*/".contains(element) {
    //             println!("stack: {:?}", stack);
    //             let a = stack.pop().expect("Failed to get number a");
    //             let b = stack.pop().expect("Failed to get number b");
    //             let result: u32 = match element {
    //                 '+' => a + b,
    //                 '-' => a.saturating_sub(b),
    //                 '*' => a * b,
    //                 '/' => a / b,
    //                 _ => {
    //                     println!("Invalid operator");
    //                     0
    //                 }
    //             };
    //             stack.push(result);
    //         }
    //         println!("output stack {:?}", input);
    //         println!("calculation stack {:?}", stack);
    //     }
    //     stack.pop().expect("Failed to get final result")
    // }
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
    let exp = String::from("30+4-2");
    expression.expression_to_stack(&exp);
}
