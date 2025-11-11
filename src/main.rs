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
                        .expect("Failed to fetch number probably because of wrongly set big flag")
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
                big = false;
            }
        }
    }
    fn shunting_yard(&mut self) {
        let mut operator_stack: Vec<Opts> = Vec::new();
        let mut output_stack: Vec<Opts> = Vec::new();
        for element in self.stack.drain(..) {
            // case operator
            if !matches!(element, Opts::Leftpar | Opts::Rightpar | Opts::Num(_)) {
                // while precedence of the current token (element) is smaller or equal to the operator on
                // the operator stack, it pushes the headof the operator stack to the output
                while !operator_stack.is_empty()
                    && precedence(&element) <= precedence(operator_stack.last().unwrap())
                {
                    output_stack.push(operator_stack.pop().unwrap());
                }
                operator_stack.push(element);

            // case paranthesis
            } else if matches!(element, Opts::Leftpar) {
                operator_stack.push(element);
            } else if matches!(element, Opts::Rightpar) {
                while !matches!(
                    operator_stack
                        .last()
                        .expect("Unvalid paranthesis placement"),
                    Opts::Leftpar
                ) {
                    output_stack.push(operator_stack.pop().unwrap());
                }
                operator_stack.pop();
            // case number
            } else {
                output_stack.push(element);
            }
        }
        // moving the remaining elements onto the output stack
        while let Some(element) = operator_stack.pop() {
            output_stack.push(element);
        }
        self.stack = output_stack;
    }
    fn solve_rpn(&mut self) -> u32 {
        let mut number_stack: Vec<Opts> = Vec::new();
        for element in self.stack.drain(..) {
            if matches!(element, Opts::Num(_)) {
                number_stack.push(element);
            } else {
                let b = number_stack
                    .pop()
                    .expect("Failed to get number b")
                    .get_num();
                let a = number_stack
                    .pop()
                    .expect("Failed to get number a")
                    .get_num();
                let result: Opts = match element {
                    Opts::Plus => Opts::Num(a + b),
                    Opts::Minus => Opts::Num(a - b),
                    Opts::Multiply => Opts::Num(a * b),
                    Opts::Divide => Opts::Num(a / b),
                    _ => {
                        println!("Invalid Operator");
                        Opts::Num(0)
                    }
                };
                number_stack.push(result);
            }
        }
        number_stack.pop().expect("Failed to get Result").get_num()
    }
}
fn precedence(op: &Opts) -> u8 {
    match op {
        Opts::Plus | Opts::Minus => 1,
        Opts::Multiply | Opts::Divide => 2,
        _ => 0,
    }
}

fn main() {
    let mut expression = ExpStack::new();
    let exp = String::from("10+5*3");
    expression.expression_to_stack(&exp);
    expression.shunting_yard();
    println!("{:?}", expression.stack);
    let result = expression.solve_rpn();
    println!("{}", result)
}
