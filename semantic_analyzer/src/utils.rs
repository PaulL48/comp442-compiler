use std::fmt;

pub fn separated_list<T: fmt::Display>(list: &[T], separator: &str) -> String {
    let mut result = "".to_string();
    if list.is_empty() {
        return result;
    }

    result.push_str(&format!("{}", list.first().unwrap()));
    for element in list.iter().skip(1) {
        result.push_str(&format!("{}{}", separator, element));
    }
    result
}