// An element is either plain, bold or italic
#[derive(Debug, Clone)]
pub enum Element {
    PlainText(String),
    Bold(String),
    Italic(String),
}

// block-level element is either title, a paragraph, or a code block
#[derive(Debug, Clone)]
pub enum Block {
    Title { level: u8, content: Vec<Element> },
    Paragraph(Vec<Element>),
    CodeBlock(String),
}

// The whole file are blocks
#[derive(Debug, Clone)]
pub struct Document {
    pub blocks: Vec<Block>,
}

impl Document {
    pub fn new() -> Self {
        Document { blocks: Vec::new() }
    }
}
