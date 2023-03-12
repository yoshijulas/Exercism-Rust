#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for inp in inputs {
        if let CalculatorInput::Value(num) = *inp {
            stack.push(num);
        } else {
            let Some(num2) = stack.pop() else { return None;

            };

            let Some(num1) = stack.pop() else { return None; };

            match *inp {
                CalculatorInput::Add => stack.push(num1 + num2),
                CalculatorInput::Subtract => stack.push(num1 - num2),
                CalculatorInput::Multiply => stack.push(num1 * num2),
                CalculatorInput::Divide => stack.push(num1 / num2),
                CalculatorInput::Value(_) => unreachable!(),
            }
        }
    }
    // (stack.len() == 1).then(|| stack[0])
    let result = stack.pop();
    if stack.is_empty() {
        result
    } else {
        None
    }
}
