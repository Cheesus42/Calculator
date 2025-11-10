// 1+2+3 = 12+3+ -> 33+ -> 6
// (1+2)*3 = 12+3*
// 4*5+(2+3) = 23+45*+
//
#[derive(Debug)]
struct ExpStack {
    push_stack: Vec<String>,
    operator_stack: String,
    output_stack: Vec<String>,
}
impl ExpStack {
    // turning an expression in infix type into a vector of infix type with each element containing
    // an element
    fn new() -> Self {
        ExpStack {
            push_stack: Vec::new(),
            operator_stack: String::new(),
            output_stack: Vec::new(),
        }
    }
    fn expression_to_stack(&mut self, expression: &String) {
        for m in expression.chars() {
            if m == ' ' {
                continue;
            }
            if matches!(m, '+' | '-' | '*' | '/' | '(' | ')' | '[' | ']') {
                self.push_stack.push(m.to_string());
                continue;
            }
            m.to_string()
                .parse::<u32>()
                .expect("Not a Valid expression. Allowed expressions: +, -, *, /, (, ), [, ]");
            self.push_stack.push(m.to_string());
        }
    }
}

fn main() {
    let mut expression = ExpStack::new();
    let exp = String::from("(  1+2)*3");
    expression.expression_to_stack(&exp);
    println!("{:?}", expression.push_stack)
}
