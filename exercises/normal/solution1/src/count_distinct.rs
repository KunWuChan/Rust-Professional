pub fn new_count_distinct(input_str: &str) -> usize {
    let mut elements: Vec<&str> = input_str.split(',').filter(|s| !s.is_empty()).collect();
    if elements.is_empty() { return 0; }
    elements.sort_unstable();
    let mut count = 1;
    for i in 1..elements.len() {
        if elements[i] != elements[i - 1] {
            count += 1;
        }
    }
    count
}
