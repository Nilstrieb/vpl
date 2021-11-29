use crate::lexer::tokens::{Token, TokenKind};

pub fn compute_keyword(identifier: &str) -> Option<TokenKind> {
    Some(match identifier {
        "please" => TokenKind::Please,
        "as" => TokenKind::As,
        "add" => TokenKind::Add,
        "sub" => TokenKind::Sub,
        "mul" => TokenKind::Mul,
        "div" => TokenKind::Div,
        "mod" => TokenKind::Mod,
        "initialize" => TokenKind::Initialize,
        "variable" => TokenKind::Variable,
        "with" => TokenKind::With,
        "the" => TokenKind::The,
        "value" => TokenKind::Value,
        "of" => TokenKind::Of,
        "set" => TokenKind::Set,
        "to" => TokenKind::To,
        "from" => TokenKind::From,
        "by" => TokenKind::By,
        "take" => TokenKind::Take,
        "end" => TokenKind::End,
        "check" => TokenKind::Check,
        "whether" => TokenKind::Whether,
        "then" => TokenKind::Then,
        "do" => TokenKind::Do,
        "otherwise" => TokenKind::Otherwise,
        "break" => TokenKind::Break,
        "out" => TokenKind::Out,
        "this" => TokenKind::This,
        "create" => TokenKind::Create,
        "function" => TokenKind::Function,
        "parameter" => TokenKind::Parameter,
        "parameters" => TokenKind::Parameters,
        "that" => TokenKind::That,
        "returns" => TokenKind::Returns,
        "call" => TokenKind::Call,
        "no" => TokenKind::No,
        "argument" => TokenKind::Argument,
        "arguments" => TokenKind::Arguments,
        "and" => TokenKind::And,
        "go" => TokenKind::Go,
        "sleep" => TokenKind::Sleep,
        "absent" => TokenKind::Absent,
        "null" => TokenKind::Null,
        "noValue" => TokenKind::NoValue,
        "undefined" => TokenKind::Undefined,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "does" => TokenKind::Does,
        "has" => TokenKind::Has,
        "is" => TokenKind::Is,
        "not" => TokenKind::Not,
        "have" => TokenKind::Have,
        "greater" => TokenKind::Greater,
        "less" => TokenKind::Less,
        "than" => TokenKind::Than,
        "equal" => TokenKind::Equal,
        "or" => TokenKind::Or,
        "repeat" => TokenKind::Repeat,
        "return" => TokenKind::Return,
        _ => return None,
    })
}
