// Given a data type and dimension list print the type signature
// Ex. int[5][4]
pub fn type_string(data_type: &str, dimensions: &[i64]) -> String {
    let mut result = String::new();
    result.push_str(data_type);
    for dimension in dimensions {
        result.push_str(&format!("[{}]", dimension));
    }
    result
}

/// Return the string representation of a type including optionally defined array dimensions
/// The name parameter_ is because parameters are the only location where this optionality is allowed
/// Ex. int[][], string[1][2]
pub fn parameter_type_string(data_type: &str, dimensions: &[Option<i64>]) -> String {
    let mut result = String::new();
    result.push_str(data_type);
    for dimension in dimensions {
        if let Some(dimension) = dimension {
            result.push_str(&format!("[{}]", dimension));
        } else {
            result.push_str("[]");
        }
    }
    result
}
