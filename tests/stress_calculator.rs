use calculator::calculator::eval;
use rand::{Rng, rng};
use std::time::Instant; // adjust to your actual crate path

const MAX_DEPTH: usize = 10; // maximum nesting of parentheses
const NUM_TESTS: usize = 100000; // number of random expressions to generate

fn random_operator() -> &'static str {
    let ops = ["+", "-", "*", "/", "%"];
    let idx = rng().random_range(0..ops.len());
    ops[idx]
}

fn random_number() -> String {
    let n = rng().random_range(0..1000);
    n.to_string()
}

fn generate_random_expression(depth: usize) -> String {
    if depth > MAX_DEPTH || rng().random::<f32>() < 0.2 {
        return random_number();
    }

    let mut expr = String::new();

    // Optionally add parentheses around subexpressions
    if rng().random::<f32>() < 0.3 {
        expr.push('(');
        expr.push_str(&generate_random_expression(depth + 1));
        expr.push(')');
    } else {
        expr.push_str(&generate_random_expression(depth + 1));
    }

    // Add an operator and another subexpression with some probability
    if rng().random::<f32>() < 0.7 {
        expr.push_str(random_operator());
        expr.push_str(&generate_random_expression(depth + 1));
    }

    expr
}

#[test]
fn stress_test_large_expressions() {
    println!("Running stress test with {} expressions...", NUM_TESTS);
    let start = Instant::now();

    for i in 0..NUM_TESTS {
        let expr = generate_random_expression(0);

        // Evaluate and check for correctness or panics
        match eval(&expr) {
            Ok(_result) => {} // Optionally verify result or print for debugging
            Err(e) => {
                println!("Expression {} failed: {}\n{}", i + 1, e, expr);
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "✅ Stress test completed in {:.2?} for {} expressions",
        duration, NUM_TESTS
    );
}
