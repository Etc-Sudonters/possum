use crate::document::DocumentPosition;
use crate::project::{Project, ProjectEntry};
use crate::scavenge::ParseFailure;
use std::fmt::Display;

pub struct OneLineRender(pub Project);

impl Display for OneLineRender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.0.entries() {
            match entry {
                ProjectEntry::ParseFailure(path, err) => {
                    writeln!(f, "{}:: {}", path.display(), OneLineParseFailure(err))?;
                }
                ProjectEntry::Workflow {
                    source,
                    document,
                    annotations,
                    workflow,
                } => {
                    for a in annotations.entries() {
                        writeln!(
                            f,
                            "{}:{}: {}",
                            source.display(),
                            OneLineDocumentPosition(document.pos(&a).unwrap()),
                            &a
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}

struct OneLineDocumentPosition(DocumentPosition);

impl Display for OneLineDocumentPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0.line(), self.0.col())
    }
}

struct OneLineParseFailure<'a>(&'a ParseFailure);

impl<'a> Display for OneLineParseFailure<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.0 {
            ParseFailure::Empty => "document is empty".to_owned(),
            ParseFailure::CouldntOpen => "could not open document".to_owned(),
            ParseFailure::InvalidDocument(e) => e.to_string(),
            ParseFailure::NotAMap(_) => "root node must be a map".to_owned(),
            ParseFailure::TooManyDocuments(_) => "too many documents in file".to_owned(),
        };

        writeln!(f, "{}", msg)
    }
}
