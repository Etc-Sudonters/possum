#[derive(Debug, Copy, Clone)]
pub struct DocumentPointer(pub(crate) u64);

pub struct DocumentPosition(usize, usize);

pub struct Document {
    lines: Vec<Vec<u8>>,
    total_length: usize,
}

impl Document {
    pub fn new() -> Document {
        Document {
            lines: vec![],
            total_length: 0,
        }
    }
}
