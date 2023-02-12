#[derive(Debug, Copy, Clone)]
pub struct DocumentPointer(pub(crate) usize);

pub struct DocumentPosition(usize, usize);

pub enum DocumentError {
    OutOfBounds,
}

pub struct Document<'a> {
    raw: &'a [u8],
    total_length: usize,
}

impl<'a> Document<'a> {
    pub fn new(raw: &'a [u8]) -> Document {
        Document {
            raw,
            total_length: raw.len(),
        }
    }

    pub fn pos(&self, p: &DocumentPointer) -> Result<DocumentPosition, DocumentError> {
        let mut pos = p.0;
        //TODO this is easily pre-calculatable
        for (line, len) in self
            .raw
            .split(|c| *c == b'\n')
            .map(|l| l.len() + 1)
            .enumerate()
        {
            if len > pos {
                return Ok(DocumentPosition(line, pos));
            } else {
                pos -= len
            }
        }

        return Err(DocumentError::OutOfBounds);
    }
}
