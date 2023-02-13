use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub struct DocumentPointer(pub(crate) usize);

impl From<u64> for DocumentPointer {
    fn from(value: u64) -> Self {
        // technically, this is maybe a lossy conversion
        // honestly, I think possum would become unusable
        // before that happens
        DocumentPointer(value as usize)
    }
}

pub struct DocumentPosition(usize, usize);

pub enum DocumentError {
    OutOfBounds,
}

#[derive(Debug)]
pub struct Document {
    raw: Vec<u8>,
    len: usize,
    lines: Vec<usize>,
}

impl Document {
    pub fn new<D>(raw: D) -> Document
    where
        D: Into<Vec<u8>>,
    {
        let raw = raw.into();
        let len = raw.len();
        let lines = raw.split(|c| *c == b'\n').map(|l| l.len() + 1).collect();
        Document { raw, len, lines }
    }

    pub fn pos(&self, p: &DocumentPointer) -> Result<DocumentPosition, DocumentError> {
        let mut pos = p.0;
        for (line, len) in self.lines.iter().enumerate() {
            if *len > pos {
                return Ok(DocumentPosition(line, pos));
            } else {
                pos -= len
            }
        }

        return Err(DocumentError::OutOfBounds);
    }
}
