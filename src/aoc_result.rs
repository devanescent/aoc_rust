use std::fmt;

#[allow(dead_code)]
pub enum AoCResult {
    Num(u64),
    Str(String),
}

impl std::fmt::Display for AoCResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AoCResult::Num(n) => write!(f, "{}", n),
            AoCResult::Str(s) => write!(f, "{}", s),
        }
    }
}

impl From<AoCResult> for u64 {
    fn from(res: AoCResult) -> Self {
        if let AoCResult::Num(n) = res { n } else { 0 }
    }
}

impl From<AoCResult> for String {
    fn from(res: AoCResult) -> Self {
        if let AoCResult::Str(s) = res {
            s
        } else {
            String::from("")
        }
    }
}
