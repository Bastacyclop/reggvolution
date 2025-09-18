use crate::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub struct Index(pub u32);

impl std::str::FromStr for Index {
  type Err = Option<std::num::ParseIntError>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with("%") {
      s["%".len()..].parse().map(Index).map_err(Some)
    } else {
      Err(None)
    }
  }
}

impl std::fmt::Display for Index {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "%{}", self.0)
  }
}

define_language! {
  pub enum Rise {
    Var(Index),
    "app" = App([Id; 2]),
    "lam" = Lambda(Id),

    // to implement explicit substitution:
    // "sig" = Sigma([Id; 3]),
    // "phi" = Phi([Id; 3]),

    Number(i32),
    Symbol(Symbol),
  }
}

#[derive(Default)]
pub struct RiseAnalysis;

type AnalysisData = ();

impl Analysis<Rise> for RiseAnalysis {
  type Data = AnalysisData;

  fn merge(&mut self, to: &mut AnalysisData, from: AnalysisData) -> DidMerge {
    DidMerge(false, false)
  }

  fn make(egraph: &RiseEGraph, enode: &Rise) -> AnalysisData {
    ()
  }
}