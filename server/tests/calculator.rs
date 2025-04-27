// use rig::rig_tool;
use rig_derive::rig_tool;

#[rig_tool(
    description = "Perform basic arithmetic operations",
    params(
        x = "First number in the calculation",
        y = "Second number in the calculation",
        operation = "The operation to perform (add, subtract, multiply, divide)"
    )
)]
async fn calculator(x: i32, y: i32, operation: String) -> Result<i32, rig::tool::ToolError> {
    match operation.as_str() {
        "add" => Ok(x + y),
        "subtract" => Ok(x - y),
        "multiply" => Ok(x * y),
        "divide" => {
            if y == 0 {
                Err(rig::tool::ToolError::ToolCallError(
                    "Division by zero".into(),
                ))
            } else {
                Ok(x / y)
            }
        }
        _ => Err(rig::tool::ToolError::ToolCallError(
            format!("Unknown operation: {}", operation).into(),
        )),
    }
}