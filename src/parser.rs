use crate::data::{Block, Document, Element};

#[derive(Debug)]
enum LineType {
    Title { level: u8, content: String },
    PlainText(String),
    Empty,
}

pub fn parse_markdown(text: &str) -> Result<Document, String> {
    let mut blocks = Vec::new();
    let mut current_paragraph: Option<Vec<String>> = None;

    // input based on lines
    for line in text.lines() {
        let line_type = parse_line(line);

        match line_type {
            LineType::Empty => {
                if let Some(paragraph_lines) = current_paragraph.take() {
                    let combined_text = paragraph_lines.join("");
                    let elements = parse_inline(&combined_text);
                    blocks.push(Block::Paragraph(elements));
                }
            }
            LineType::Title { level, content } => {
                if let Some(paragraph_lines) = current_paragraph.take() {
                    let combined_text = paragraph_lines.join("");
                    let elements = parse_inline(&combined_text);
                    blocks.push(Block::Paragraph(elements));
                }

                let title_elements = parse_inline(&content);
                blocks.push(Block::Title {
                    level,
                    content: title_elements,
                });
            }
            LineType::PlainText(content) => match &mut current_paragraph {
                Some(paragraph_lines) => {
                    paragraph_lines.push(content);
                }
                None => {
                    current_paragraph = Some(vec![content]);
                }
            },
        }
    }
    // the last paragraph line
    if let Some(paragraph_lines) = current_paragraph {
        let combined_text = paragraph_lines.join("");
        let elements = parse_inline(&combined_text);
        blocks.push(Block::Paragraph(elements));
    }

    Ok(Document { blocks })
}

fn parse_line(line: &str) -> LineType {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return LineType::Empty;
    }

    if trimmed.starts_with('#') {
        let level = trimmed.chars().take_while(|&c| c == '#').count() as u8;

        if level > 0 && level <= 6 {
            let content = trimmed
                .chars()
                .skip(level as usize)
                .collect::<String>()
                .trim()
                .to_string();

            return LineType::Title { level, content };
        }
    }

    LineType::PlainText(trimmed.to_string())
}

fn parse_inline(text: &str) -> Vec<Element> {
    let mut elements = Vec::new();
    let mut current_pos = 0;
    let chars: Vec<char> = text.chars().collect();

    while current_pos < chars.len() {
        if current_pos + 1 < chars.len()
            && (chars[current_pos] == '*')
            && chars[current_pos + 1] == '*'
        {
            if let Some(end_pos) = find_closing_pattern(&chars, current_pos + 2, "**") {
                let bold_content: String = chars[(current_pos + 2)..end_pos].iter().collect();
                elements.push(Element::Bold(bold_content));
                current_pos = end_pos + 2;
            } else {
                elements.push(Element::PlainText("*".to_string()));
                current_pos += 1;
            }
        } else if chars[current_pos] == '*' {
            if let Some(end_pos) = find_closing_pattern(&chars, current_pos + 1, "*") {
                let italic_content: String = chars[(current_pos + 1)..end_pos].iter().collect();
                elements.push(Element::Italic(italic_content));
                current_pos = end_pos + 1;
            }
        } else {
            let start_pos = current_pos;
            while current_pos < chars.len() && chars[current_pos] != '*' {
                current_pos += 1
            }

            if start_pos < current_pos {
                let plain_text: String = chars[start_pos..current_pos].iter().collect();
                elements.push(Element::PlainText(plain_text));
            }
        }
    }

    if elements.is_empty() {
        elements.push(Element::PlainText(text.to_string()));
    }
    elements
}

fn find_closing_pattern(chars: &[char], start_pos: usize, pattern: &str) -> Option<usize> {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let pattern_len = pattern_chars.len();

    for i in start_pos..=chars.len().saturating_sub(pattern_len) {
        if chars[i..i + pattern_len] == pattern_chars {
            return Some(i);
        }
    }
    None
}
