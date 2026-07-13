# `stdlib/collections`

The collections package is a pure-Neu dependency built on `stdlib/core`.
The initial source surface declares the nominal `Vector<T>`, `Slice<T>`, and
`Iterator<T>` types. Their operations remain gated by the accepted generic,
borrow, protocol, iterator, and allocation-failure contracts in the ADRs.
