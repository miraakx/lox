use rustc_hash::FxHashMap;
use string_interner::symbol::SymbolU32;

pub type IdentifierSymbol = SymbolU32;
pub type ExprId = i64;
pub type SideTable = FxHashMap<ExprId, usize>;