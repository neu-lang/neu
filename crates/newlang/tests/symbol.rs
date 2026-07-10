use newlang::module::ModuleName;
use newlang::symbol::{
    NameTable, NameTableEntry, NameTableInsert, NameTableKey, SymbolId, SymbolInterner,
};

#[test]
fn same_text_interns_to_same_symbol_id() {
    let mut interner = SymbolInterner::new();

    let first = interner.intern("Demo");
    let second = interner.intern("Demo");

    assert_eq!(first, second);
    assert_eq!(first.index(), 0);
    assert_eq!(interner.symbols(), ["Demo"]);
}

#[test]
fn different_text_gets_distinct_stable_ids() {
    let mut interner = SymbolInterner::new();

    let first = interner.intern("alpha");
    let second = interner.intern("beta");
    let third = interner.intern("alpha");

    assert_eq!(first.index(), 0);
    assert_eq!(second.index(), 1);
    assert_eq!(third, first);
    assert_ne!(first, second);
}

#[test]
fn unknown_symbol_ids_do_not_resolve() {
    let interner = SymbolInterner::new();

    assert_eq!(interner.resolve(SymbolId::from_raw(99)), None);
}

#[test]
fn symbols_preserve_exact_text_and_insertion_order() {
    let mut interner = SymbolInterner::new();

    let lower = interner.intern("name");
    let upper = interner.intern("Name");
    let dotted = interner.intern("pkg.name");

    assert_eq!(interner.resolve(lower), Some("name"));
    assert_eq!(interner.resolve(upper), Some("Name"));
    assert_eq!(interner.resolve(dotted), Some("pkg.name"));
    assert_eq!(interner.symbols(), ["name", "Name", "pkg.name"]);
}

#[test]
fn same_text_can_exist_in_distinct_modules() {
    let mut interner = SymbolInterner::new();
    let symbol = interner.intern("Thing");
    let first_module = ModuleName::parse("demo.one").unwrap();
    let second_module = ModuleName::parse("demo.two").unwrap();
    let mut table = NameTable::new();

    let first = table.insert(NameTableEntry::new(
        NameTableKey::new(first_module.clone(), symbol),
        "first declaration",
    ));
    let second = table.insert(NameTableEntry::new(
        NameTableKey::new(second_module.clone(), symbol),
        "second declaration",
    ));

    assert!(matches!(first, NameTableInsert::Inserted(_)));
    assert!(matches!(second, NameTableInsert::Inserted(_)));
    assert_eq!(
        table
            .get(&NameTableKey::new(first_module, symbol))
            .unwrap()
            .value(),
        "first declaration"
    );
    assert_eq!(
        table
            .get(&NameTableKey::new(second_module, symbol))
            .unwrap()
            .value(),
        "second declaration"
    );
}

#[test]
fn name_table_lookup_uses_exact_module_and_symbol() {
    let mut interner = SymbolInterner::new();
    let alpha = interner.intern("alpha");
    let beta = interner.intern("beta");
    let module = ModuleName::parse("demo").unwrap();
    let mut table = NameTable::new();

    table.insert(NameTableEntry::new(
        NameTableKey::new(module.clone(), alpha),
        "alpha declaration",
    ));

    assert!(table
        .get(&NameTableKey::new(module.clone(), alpha))
        .is_some());
    assert!(table.get(&NameTableKey::new(module, beta)).is_none());
}

#[test]
fn duplicate_insert_reports_existing_entry_without_replacing() {
    let mut interner = SymbolInterner::new();
    let symbol = interner.intern("Thing");
    let module = ModuleName::parse("demo").unwrap();
    let key = NameTableKey::new(module, symbol);
    let mut table = NameTable::new();

    let inserted = table.insert(NameTableEntry::new(key.clone(), "original"));
    let duplicate = table.insert(NameTableEntry::new(key.clone(), "replacement"));

    assert!(matches!(inserted, NameTableInsert::Inserted(_)));
    match duplicate {
        NameTableInsert::Duplicate {
            existing,
            attempted,
        } => {
            assert_eq!(existing.value(), "original");
            assert_eq!(attempted.value(), "replacement");
        }
        NameTableInsert::Inserted(_) => panic!("duplicate insert should not be inserted"),
    }
    assert_eq!(table.get(&key).unwrap().value(), "original");
    assert_eq!(table.entries().len(), 1);
}

#[test]
fn missing_name_table_key_returns_none() {
    let key = NameTableKey::new(ModuleName::parse("demo").unwrap(), SymbolId::from_raw(0));
    let table = NameTable::new();

    assert!(table.get(&key).is_none());
}
