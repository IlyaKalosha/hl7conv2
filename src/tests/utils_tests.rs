use crate::utils;

#[test]
fn test_replace_eof_windows() {
    let input = "line1\r\nline2\r\nline3\r\n";
    let expected = "line1\nline2\nline3\n";
    assert_eq!(utils::replace_eof(input.to_string()), expected);
}

#[test]
fn test_replace_eof_mac() {
    let input = "line1\rline2\rline3\r";
    let expected = "line1\nline2\nline3\n";
    assert_eq!(utils::replace_eof(input.to_string()), expected);
}

#[test]
fn test_replace_eof_mixed() {
    let input = "line1\n\rline2\r\nline3\r";
    let expected = "line1\nline2\nline3\n";
    assert_eq!(utils::replace_eof(input.to_string()), expected);
}

#[test]
fn test_replace_eof_unix() {
    let input = "line1\nline2\nline3\n";
    let expected = "line1\nline2\nline3\n";
    assert_eq!(utils::replace_eof(input.to_string()), expected);
}

#[test]
fn test_split_segments() {
    let input = "MSH|field1|field2\nEVN|field1|field2\nPID|field1|field2".to_string();
    let expected = vec![
        "MSH|field1|field2".to_string(),
        "EVN|field1|field2".to_string(),
        "PID|field1|field2".to_string(),
    ];
    assert_eq!(utils::split_segments(input), expected);
}

#[test]
fn test_split_segments_empty() {
    let input = "".to_string();
    let expected = vec!["".to_string()];
    assert_eq!(utils::split_segments(input), expected);
}

#[test]
fn test_split_segments_single_line() {
    let input = "MSH|field1|field2".to_string();
    let expected = vec!["MSH|field1|field2".to_string()];
    assert_eq!(utils::split_segments(input), expected);
}
