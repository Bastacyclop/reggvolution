pub(crate) use egg::*;

mod lang;

pub use lang::{Rise, RiseAnalysis};
pub type RiseEGraph = EGraph<Rise, RiseAnalysis>;
pub type RiseExpr = RecExpr<Rise>;