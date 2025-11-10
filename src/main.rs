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
    fn expression_to_stack(&mut self, expression: String) {
        for m in expression.chars() {
            if m == ' ' {
                continue;
            }
            if matches!(m, '+' | '-' | '*' | '/' | '(' | ')' | '[' | ']') {
                self.push_stack.push(m.to_string());
                continue;
            }
            let parsed_m: u32 = m
                .to_string()
                .parse()
                .expect("Not a Valid expression allowed expressions: +, -, *, /, (, ), [, ]");
        }
    }
}

fn main() {}

fn string_to_expression() {}
