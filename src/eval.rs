use crate::{AstNode, Operation};

pub fn evaluate_simple(node: AstNode) -> Result<f64, ()> {
    match node {
        AstNode::Expression { lhs, rhs, operator } => match operator {
            Operation::Add => Ok(lhs + rhs),
            Operation::Subtract => Ok(lhs - rhs),
            Operation::Divide => Ok(lhs / rhs),
            Operation::Multiply => Ok(lhs * rhs),
            Operation::Exponent => Ok(lhs.powf(rhs)),
            _ => Err(()),
        },
        _ => Err(()),
    }
}
