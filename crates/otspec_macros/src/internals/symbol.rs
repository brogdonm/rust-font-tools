use std::fmt::{self, Display};
use syn::{Ident, Path};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

pub const BORROW: Symbol = Symbol("borrow");
pub const BOUND: Symbol = Symbol("bound");
pub const CONTENT: Symbol = Symbol("content");
pub const CRATE: Symbol = Symbol("crate");
pub const DEFAULT: Symbol = Symbol("default");
pub const DENY_UNKNOWN_FIELDS: Symbol = Symbol("deny_unknown_fields");
pub const DESERIALIZE: Symbol = Symbol("deserialize");
pub const DESERIALIZE_WITH: Symbol = Symbol("deserialize_with");
pub const FIELD_IDENTIFIER: Symbol = Symbol("field_identifier");
pub const OTHER: Symbol = Symbol("other");
pub const OFFSET_BASE: Symbol = Symbol("offset_base");
pub const REMOTE: Symbol = Symbol("remote");
pub const RENAME: Symbol = Symbol("rename");
pub const SERDE: Symbol = Symbol("serde");
pub const SERIALIZE: Symbol = Symbol("serialize");
pub const SERIALIZE_WITH: Symbol = Symbol("serialize_with");
pub const SKIP_DESERIALIZING: Symbol = Symbol("skip_deserializing");
pub const SKIP_SERIALIZING: Symbol = Symbol("skip_serializing");
pub const TAG: Symbol = Symbol("tag");
pub const UNTAGGED: Symbol = Symbol("untagged");
pub const VARIANT_IDENTIFIER: Symbol = Symbol("variant_identifier");
pub const WITH: Symbol = Symbol("with");
pub const EXPECTING: Symbol = Symbol("expecting");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
