const ADDR_SIZE: usize = 4; // When passing arrays or string around (???)

pub fn size_of_optional(data_type: &str, dimensions: &Vec<Option<i64>>) -> usize {
    if dimensions.len() == 0 {
        return base_size_of(data_type);
    } else {
        return ADDR_SIZE;
    }
}

pub fn size_of(data_type: &str, dimensions: &Vec<i64>) -> usize {
    let mut size = base_size_of(data_type);
    for dimension in dimensions {
        size *= *dimension as usize;
    }
    size
}

fn base_size_of(data_type: &str) -> usize {
    match data_type {
        "integer" => 4,
        "float" => 4,
        "string" => ADDR_SIZE,
        _ => panic!(),
    }
}
