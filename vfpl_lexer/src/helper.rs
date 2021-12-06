use crate::tokens::CondKeyword;
use crate::TokenKind;

pub fn compute_keyword(identifier: &str) -> Option<TokenKind> {
    Some(match identifier {
        "please" => TokenKind::Please,
        "as" => TokenKind::As,
        "initialize" => TokenKind::Initialize,
        "variable" => TokenKind::Variable,
        "end" => TokenKind::End,
        "check" => TokenKind::Check,
        "whether" => TokenKind::Whether,
        "then" => TokenKind::Then,
        "do" => TokenKind::Do,
        "otherwise" => TokenKind::Otherwise,
        "break" => TokenKind::Break,
        "this" => TokenKind::This,
        "create" => TokenKind::Create,
        "function" => TokenKind::Function,
        "call" => TokenKind::Call,
        "and" => TokenKind::And,
        "absent" => TokenKind::Absent,
        "null" => TokenKind::Null,
        "novalue" => TokenKind::NoValue,
        "undefined" => TokenKind::Undefined,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "not" => TokenKind::Not,
        "or" => TokenKind::Or,
        "repeat" => TokenKind::Repeat,
        "return" => TokenKind::Return,
        "while" => TokenKind::While,
        "add" => TokenKind::CondKeyword(CondKeyword::Add),
        "sub" => TokenKind::CondKeyword(CondKeyword::Sub),
        "mul" => TokenKind::CondKeyword(CondKeyword::Mul),
        "div" => TokenKind::CondKeyword(CondKeyword::Div),
        "modulo" => TokenKind::CondKeyword(CondKeyword::Mod),
        "with" => TokenKind::CondKeyword(CondKeyword::With),
        "the" => TokenKind::CondKeyword(CondKeyword::The),
        "value" => TokenKind::CondKeyword(CondKeyword::Value),
        "of" => TokenKind::CondKeyword(CondKeyword::Of),
        "set" => TokenKind::CondKeyword(CondKeyword::Set),
        "to" => TokenKind::CondKeyword(CondKeyword::To),
        "from" => TokenKind::CondKeyword(CondKeyword::From),
        "by" => TokenKind::CondKeyword(CondKeyword::By),
        "take" => TokenKind::CondKeyword(CondKeyword::Take),
        "out" => TokenKind::CondKeyword(CondKeyword::Out),
        "parameter" => TokenKind::CondKeyword(CondKeyword::Parameter),
        "parameters" => TokenKind::CondKeyword(CondKeyword::Parameters),
        "that" => TokenKind::CondKeyword(CondKeyword::That),
        "returns" => TokenKind::CondKeyword(CondKeyword::Returns),
        "no" => TokenKind::CondKeyword(CondKeyword::No),
        "argument" => TokenKind::CondKeyword(CondKeyword::Argument),
        "arguments" => TokenKind::CondKeyword(CondKeyword::Arguments),
        "go" => TokenKind::CondKeyword(CondKeyword::Go),
        "sleep" => TokenKind::CondKeyword(CondKeyword::Sleep),
        "does" => TokenKind::CondKeyword(CondKeyword::Does),
        "has" => TokenKind::CondKeyword(CondKeyword::Has),
        "is" => TokenKind::CondKeyword(CondKeyword::Is),
        "have" => TokenKind::CondKeyword(CondKeyword::Have),
        "greater" => TokenKind::CondKeyword(CondKeyword::Greater),
        "less" => TokenKind::CondKeyword(CondKeyword::Less),
        "than" => TokenKind::CondKeyword(CondKeyword::Than),
        "equal" => TokenKind::CondKeyword(CondKeyword::Equal),
        _ => return None,
    })
}
