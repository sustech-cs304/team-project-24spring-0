use regex::Regex;
pub fn lines_count(content: &str) -> usize {
    let mut line_count: usize = 0;
    for c in content.chars() {
        if c == '\n' {
            line_count += 1;
        }
    }
    line_count
}

pub fn all_to_lf(content: &str) -> String {
    let re = Regex::new(r"\r\n|\r").unwrap();
    re.replace_all(content, "\n").to_string()
}
