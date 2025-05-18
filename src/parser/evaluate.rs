use super::value::{Value, ValueError};

pub trait Evaluate {
    fn evaluate(&self) -> Result<Value, ValueError>;

    fn evaluate_boolean(&self) -> Result<bool, ValueError>;
    fn evaluate_numeric(&self) -> Result<f64, ValueError>;
    fn evaluate_string(&self) -> Result<String, ValueError>;
}
