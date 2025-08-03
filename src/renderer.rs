use crate::data::{Block, Document, Element};

// Renders to render element, list of elements, block and whole document
impl Document {
    pub fn to_html(&self) -> String {
        self.blocks
            .iter()
            .map(|block| render_block(block))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn render_block(block: &Block) -> String {
    match block {
        Block::Title { level, content } => {
            let content_html = render_elements(content);
            format!("<h{}>{}</h{}>", level, content_html, level)
        }
        Block::Paragraph(element) => {
            let content_html = render_elements(element);
            format!("<p>{}</p>", content_html)
        }
        Block::CodeBlock(code) => {
            format!("<pre><code>{}</pre><code>", code)
        }
    }
}

fn render_elements(elements: &Vec<Element>) -> String {
    elements
        .iter()
        .map(|element| render_element(element))
        .collect::<Vec<String>>()
        .join("")
}

fn render_element(element: &Element) -> String {
    match element {
        Element::PlainText(text) => text.clone(),
        Element::Bold(text) => format!("<strong>{}</strong>", text),
        Element::Italic(text) => format!("<em>{}</em>", text),
    }
}

