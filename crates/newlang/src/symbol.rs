use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SymbolId(usize);

impl SymbolId {
    pub fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Default)]
pub struct SymbolInterner {
    symbols: Vec<String>,
    ids_by_text: HashMap<String, SymbolId>,
}

impl SymbolInterner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, text: &str) -> SymbolId {
        if let Some(id) = self.ids_by_text.get(text) {
            return *id;
        }

        let id = SymbolId(self.symbols.len());
        self.symbols.push(text.to_owned());
        self.ids_by_text.insert(text.to_owned(), id);
        id
    }

    pub fn resolve(&self, id: SymbolId) -> Option<&str> {
        self.symbols.get(id.index()).map(String::as_str)
    }

    pub fn symbols(&self) -> &[String] {
        &self.symbols
    }
}
