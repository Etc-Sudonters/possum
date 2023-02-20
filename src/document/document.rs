use std::convert::AsRef;
use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub struct DocumentPointer(pub(crate) usize);

impl AsDocumentPointer for DocumentPointer {
    fn as_document_pointer(&self) -> DocumentPointer {
        self.clone()
    }
}

pub trait AsDocumentPointer {
    fn as_document_pointer(&self) -> DocumentPointer;
}

impl From<u64> for DocumentPointer {
    fn from(value: u64) -> Self {
        // technically, this is maybe a lossy conversion
        // honestly, I think possum would become unusable
        // before that happens
        DocumentPointer(value as usize)
    }
}

pub struct DocumentPosition(usize, usize);

impl DocumentPosition {
    // 1 indexed
    pub fn line(&self) -> usize {
        self.0
    }

    // 1 indexed
    pub fn col(&self) -> usize {
        self.1
    }
}

#[derive(Debug)]
pub enum DocumentError {
    OutOfBounds,
}

pub struct Document {
    _raw: Vec<u8>,
    len: usize,
    lines: Vec<usize>,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "len: {:?}\t lines: {:?}", self.len, self.lines)
    }
}

impl Document {
    pub fn new<D>(raw: D) -> Document
    where
        D: Into<Vec<u8>>,
    {
        let _raw = raw.into();
        let len = _raw.len();
        let lines = _raw.split(|c| *c == b'\n').map(|l| l.len() + 1).collect();
        Document { _raw, len, lines }
    }

    pub fn pos<P>(&self, p: P) -> Result<DocumentPosition, DocumentError>
    where
        P: AsRef<DocumentPointer>,
    {
        let mut pos = p.as_ref().0;
        for (line, len) in self.lines.iter().enumerate() {
            if *len > pos {
                // this is real super duper wrong, just works because I only have ascii documents
                return Ok(DocumentPosition(line + 1, pos + 1));
            } else {
                pos -= len
            }
        }

        return Err(DocumentError::OutOfBounds);
    }
}
