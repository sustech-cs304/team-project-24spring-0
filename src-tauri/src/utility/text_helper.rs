pub fn lines_count(content: &str) -> usize {
    let mut line_count: usize = 0;
    for c in content.chars() {
        if c == '\n' {
            line_count += 1;
        }
    }
    line_count
}
