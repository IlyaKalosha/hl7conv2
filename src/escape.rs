use crate::errors::Hl7Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Hl7EscapeHandler {
    escape_sequences: HashMap<String, String>,
    field_separator: char,
    component_separator: char,
    repetition_separator: char,
    escape_character: char,
    subcomponent_separator: char,
}

impl Default for Hl7EscapeHandler {
    fn default() -> Self {
        Self::new('|', '^', '~', '\\', '&')
    }
}

impl Hl7EscapeHandler {
    pub fn new(
        field_separator: char,
        component_separator: char,
        repetition_separator: char,
        escape_character: char,
        subcomponent_separator: char,
    ) -> Self {
        let mut escape_sequences = HashMap::new();

        escape_sequences.insert("F".to_string(), field_separator.to_string());
        escape_sequences.insert("S".to_string(), component_separator.to_string());
        escape_sequences.insert("R".to_string(), repetition_separator.to_string());
        escape_sequences.insert("E".to_string(), escape_character.to_string());
        escape_sequences.insert("T".to_string(), subcomponent_separator.to_string());
        escape_sequences.insert("X0A".to_string(), "\n".to_string());
        escape_sequences.insert("X0D".to_string(), "\r".to_string());
        escape_sequences.insert("X0D0A".to_string(), "\r\n".to_string());
        escape_sequences.insert("X20".to_string(), " ".to_string());
        escape_sequences.insert("X09".to_string(), "\t".to_string());

        Self {
            escape_sequences,
            field_separator,
            component_separator,
            repetition_separator,
            escape_character,
            subcomponent_separator,
        }
    }

    pub fn from_msh_field(field_1: &str) -> Result<Self, Hl7Error> {
        if field_1.len() != 4 {
            return Err(Hl7Error::InvalidFieldSeparators {
                separators: field_1.to_string(),
            });
        }

        let chars: Vec<char> = field_1.chars().collect();
        Ok(Self::new(chars[0], chars[1], chars[2], chars[3], '&'))
    }

    pub fn unescape(&self, text: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        let escape_char = self.escape_character;

        while let Some(ch) = chars.next() {
            if ch == escape_char {
                if let Some(next_ch) = chars.next() {
                    if next_ch == escape_char {
                        result.push(escape_char);
                    } else {
                        let mut sequence = String::new();
                        sequence.push(next_ch);

                        while let Some(&ch) = chars.peek() {
                            if ch.is_alphanumeric() {
                                sequence.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }

                        if let Some(replacement) = self.escape_sequences.get(&sequence) {
                            result.push_str(replacement);
                        } else {
                            result.push(escape_char);
                            result.push_str(&sequence);
                        }

                        if let Some(&ch) = chars.peek() {
                            if ch == escape_char {
                                chars.next();
                            }
                        }
                    }
                } else {
                    result.push(escape_char);
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    pub fn escape(&self, text: &str) -> String {
        let mut result = String::new();
        let escape_char = self.escape_character;

        for ch in text.chars() {
            match ch {
                c if c == self.field_separator => {
                    result.push(escape_char);
                    result.push('F');
                    result.push(escape_char);
                }
                c if c == self.component_separator => {
                    result.push(escape_char);
                    result.push('S');
                    result.push(escape_char);
                }
                c if c == self.repetition_separator => {
                    result.push(escape_char);
                    result.push('R');
                    result.push(escape_char);
                }
                c if c == self.subcomponent_separator => {
                    result.push(escape_char);
                    result.push('T');
                    result.push(escape_char);
                }
                c if c == escape_char => {
                    result.push(escape_char);
                    result.push(escape_char);
                }
                '\n' => {
                    result.push(escape_char);
                    result.push_str("X0A");
                    result.push(escape_char);
                }
                '\r' => {
                    result.push(escape_char);
                    result.push_str("X0D");
                    result.push(escape_char);
                }
                ' ' => {
                    result.push(escape_char);
                    result.push_str("X20");
                    result.push(escape_char);
                }
                '\t' => {
                    result.push(escape_char);
                    result.push_str("X09");
                    result.push(escape_char);
                }
                _ => result.push(ch),
            }
        }

        result
    }

    pub fn parse_field_with_escaping(&self, field_text: &str) -> Vec<String> {
        let unescaped = self.unescape(field_text);
        unescaped
            .split(self.component_separator)
            .map(|s| s.to_string())
            .collect()
    }

    pub fn format_field_with_escaping(&self, components: &[String]) -> String {
        components
            .iter()
            .map(|component| self.escape(component))
            .collect::<Vec<_>>()
            .join(&self.component_separator.to_string())
    }

    pub fn get_component_separator(&self) -> char {
        self.component_separator
    }
}

pub fn create_default_escape_handler() -> Hl7EscapeHandler {
    Hl7EscapeHandler::default()
}
