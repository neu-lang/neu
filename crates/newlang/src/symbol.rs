use std::collections::HashMap;

use crate::module::ModuleName;

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NameTableKey {
    module: ModuleName,
    symbol: SymbolId,
}

impl NameTableKey {
    pub fn new(module: ModuleName, symbol: SymbolId) -> Self {
        Self { module, symbol }
    }

    pub fn module(&self) -> &ModuleName {
        &self.module
    }

    pub fn symbol(&self) -> SymbolId {
        self.symbol
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NameTableEntry {
    key: NameTableKey,
    value: String,
}

impl NameTableEntry {
    pub fn new(key: NameTableKey, value: impl Into<String>) -> Self {
        Self {
            key,
            value: value.into(),
        }
    }

    pub fn key(&self) -> &NameTableKey {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NameTableInsert {
    Inserted(NameTableEntry),
    Duplicate {
        existing: NameTableEntry,
        attempted: NameTableEntry,
    },
}

#[derive(Debug, Default)]
pub struct NameTable {
    entries: Vec<NameTableEntry>,
    indices_by_key: HashMap<NameTableKey, usize>,
}

impl NameTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, entry: NameTableEntry) -> NameTableInsert {
        if let Some(index) = self.indices_by_key.get(entry.key()) {
            return NameTableInsert::Duplicate {
                existing: self.entries[*index].clone(),
                attempted: entry,
            };
        }

        let index = self.entries.len();
        self.indices_by_key.insert(entry.key().clone(), index);
        self.entries.push(entry.clone());
        NameTableInsert::Inserted(entry)
    }

    pub fn get(&self, key: &NameTableKey) -> Option<&NameTableEntry> {
        self.indices_by_key
            .get(key)
            .and_then(|index| self.entries.get(*index))
    }

    pub fn entries(&self) -> &[NameTableEntry] {
        &self.entries
    }
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
