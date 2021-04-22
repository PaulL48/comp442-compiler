//! Defined the way that assembly labels are mangled to make them unique within a file

const MANGLE_SEPARATOR: &str = "_";
const PARAMETER_PREFIX: &str = "p";
const RETURN_PREFIX: &str = "ret";
const EXIT_PREFIX: &str = "exit";

pub fn mangle_id(id: &str, mangled_function: &str, mangled_class: Option<&str>) -> String {
    if let Some(class_scope) = mangled_class {
        format!("{}{}{}{}{}{}", MANGLE_SEPARATOR, class_scope, MANGLE_SEPARATOR, mangled_function, MANGLE_SEPARATOR, id)
    } else {
        format!("{}{}{}{}", MANGLE_SEPARATOR, mangled_function, MANGLE_SEPARATOR, id)
    }
}

pub fn mangle_function<T: AsRef<str>>(id: &str, parameters: &[T], scope: Option<&str>) -> String {
    let mut result = String::new();
    result.push_str(MANGLE_SEPARATOR);
    if let Some(scope) = scope {
        result.push_str(scope);
        result.push_str(MANGLE_SEPARATOR);
    }

    result.push_str(id);
    for parameter in parameters {
        result.push_str(MANGLE_SEPARATOR);
        result.push_str(parameter.as_ref());
    }
    result
}

pub fn function_parameter(function: &str, position: usize, class: Option<&str>) -> String {
    if let Some(class) = class {
        format!("{}{}{}{}{}{}", class, MANGLE_SEPARATOR, function, MANGLE_SEPARATOR, PARAMETER_PREFIX, position)
    } else {
        format!("{}{}{}{}", function, MANGLE_SEPARATOR, PARAMETER_PREFIX, position)
    }
}

pub fn function_return(function: &str,  class: Option<&str>) -> String {
    if let Some(class) = class {
        format!("{}{}{}{}{}", class, MANGLE_SEPARATOR, function, MANGLE_SEPARATOR, RETURN_PREFIX)
    } else {
        format!("{}{}{}", function, MANGLE_SEPARATOR, RETURN_PREFIX)
    }
}

pub fn function_exit(function: &str,  class: Option<&str>) -> String {
    if let Some(class) = class {
        format!("{}{}{}{}{}", class, MANGLE_SEPARATOR, function, MANGLE_SEPARATOR, EXIT_PREFIX)
    } else {
        format!("{}{}{}", function, MANGLE_SEPARATOR, EXIT_PREFIX)
    }
}
