pub(crate) use egg::*;

mod lang;

pub type RiseEGraph = EGraph<lang::Rise, lang::RiseAnalysis>;
pub type RiseExpr = RecExpr<lang::Rise>;