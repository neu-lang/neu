#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceFileId(usize);

impl SourceFileId {
    pub fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ByteSpan {
    file: SourceFileId,
    start: usize,
    end: usize,
}

impl ByteSpan {
    pub fn new(file: SourceFileId, start: usize, end: usize) -> Option<Self> {
        if start <= end {
            Some(Self { file, start, end })
        } else {
            None
        }
    }

    pub fn file(self) -> SourceFileId {
        self.file
    }

    pub fn start(self) -> usize {
        self.start
    }

    pub fn end(self) -> usize {
        self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Default)]
pub struct SourceDatabase {
    files: Vec<SourceFile>,
}

impl SourceDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, path: impl Into<String>, text: impl Into<String>) -> SourceFileId {
        let id = SourceFileId(self.files.len());
        self.files.push(SourceFile {
            path: path.into(),
            text: text.into(),
        });
        id
    }

    pub fn file(&self, id: SourceFileId) -> Option<&SourceFile> {
        self.files.get(id.index())
    }

    pub fn span(&self, file: SourceFileId, start: usize, end: usize) -> Option<ByteSpan> {
        let source = self.file(file)?;
        if start <= end && end <= source.text.len() {
            Some(ByteSpan { file, start, end })
        } else {
            None
        }
    }

    pub fn line_column(&self, file: SourceFileId, offset: usize) -> Option<LineColumn> {
        let source = self.file(file)?;
        if offset > source.text.len() || !source.text.is_char_boundary(offset) {
            return None;
        }

        let mut line = 1;
        let mut line_start = 0;
        for (index, byte) in source.text.bytes().enumerate() {
            if index >= offset {
                break;
            }
            if byte == b'\n' {
                line += 1;
                line_start = index + 1;
            }
        }

        Some(LineColumn {
            line,
            column: offset - line_start + 1,
        })
    }
}

#[derive(Debug)]
pub struct SourceFile {
    path: String,
    text: String,
}

impl SourceFile {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_ids_are_stable_in_insertion_order() {
        let mut db = SourceDatabase::new();

        let first = db.add_file("first.nl", "");
        let second = db.add_file("second.nl", "value");

        assert_eq!(first.index(), 0);
        assert_eq!(second.index(), 1);
        assert_eq!(db.file(first).unwrap().path(), "first.nl");
        assert_eq!(db.file(second).unwrap().text(), "value");
    }

    #[test]
    fn empty_file_maps_offset_zero_to_first_line_first_column() {
        let mut db = SourceDatabase::new();
        let file = db.add_file("empty.nl", "");

        assert_eq!(
            db.line_column(file, 0).unwrap(),
            LineColumn { line: 1, column: 1 }
        );
    }

    #[test]
    fn single_line_ascii_offsets_map_to_one_based_columns() {
        let mut db = SourceDatabase::new();
        let file = db.add_file("single.nl", "abc");

        assert_eq!(
            db.line_column(file, 0).unwrap(),
            LineColumn { line: 1, column: 1 }
        );
        assert_eq!(
            db.line_column(file, 2).unwrap(),
            LineColumn { line: 1, column: 3 }
        );
        assert_eq!(
            db.line_column(file, 3).unwrap(),
            LineColumn { line: 1, column: 4 }
        );
    }

    #[test]
    fn multi_line_ascii_offsets_map_to_lines_and_columns() {
        let mut db = SourceDatabase::new();
        let file = db.add_file("multi.nl", "ab\ncd\n");

        assert_eq!(
            db.line_column(file, 0).unwrap(),
            LineColumn { line: 1, column: 1 }
        );
        assert_eq!(
            db.line_column(file, 3).unwrap(),
            LineColumn { line: 2, column: 1 }
        );
        assert_eq!(
            db.line_column(file, 5).unwrap(),
            LineColumn { line: 2, column: 3 }
        );
        assert_eq!(
            db.line_column(file, 6).unwrap(),
            LineColumn { line: 3, column: 1 }
        );
    }

    #[test]
    fn invalid_offsets_and_spans_are_rejected() {
        let mut db = SourceDatabase::new();
        let file = db.add_file("bounds.nl", "abc");

        assert!(db.line_column(file, 4).is_none());
        assert!(db.span(file, 0, 4).is_none());
        assert!(db.span(file, 3, 2).is_none());
        assert!(db.span(SourceFileId::from_raw(99), 0, 0).is_none());
    }
}
