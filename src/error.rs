pub fn error(line_number: usize, message: String) {
    panic!("{:?} at line {}", message, line_number)
}
