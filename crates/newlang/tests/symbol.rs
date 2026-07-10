use newlang::symbol::{SymbolId, SymbolInterner};

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
