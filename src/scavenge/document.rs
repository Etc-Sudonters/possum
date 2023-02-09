pub struct DocumentPointer(u64);
pub struct DocumentPosition(u64, u64);

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
    pub fn pos_at(self: &Self, p: &DocumentPointer) -> Result<DocumentPosition, ()> {
        todo!();
    }
}
