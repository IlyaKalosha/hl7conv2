pub fn replace_eof(hl7_string: String) -> String {
    hl7_string
        .replace("\r\n", "\n")
        .replace("\n\r", "\n")
        .replace("\r", "\n")
}

pub fn split_segments(hl7_string: String) -> Vec<String> {
    hl7_string.split('\n').map(str::to_string).collect()
}
