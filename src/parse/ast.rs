use crate::error::Span;

type Ident = String;

pub type Program = Body;

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub span: Span,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedIdent {
    pub span: Span,
    pub name: Ident,
    pub ty: Ty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarInit(VarInit),
    VarSet(VarSet),
    Add(ArithmeticOp),
    Sub(ArithmeticOp),
    Mul(ArithmeticOp),
    Div(ArithmeticOp),
    Mod(ArithmeticOp),
    If(If),
    While(While),
    FnDecl(FnDecl),
    Break(Break),
    Return(Return),
    Terminate(Terminate),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarInit {
    pub span: Span,
    pub name: TypedIdent,
    pub init: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarSet {
    pub span: Span,
    pub name: Ident,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticOp {
    pub span: Span,
    pub expr: Expr,
    pub var: Ident,
    pub kind: ArithmeticOpKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub span: Span,
    pub if_part: IfPart,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfPart {
    pub span: Span,
    pub cond: Box<Expr>,
    pub body: Body,
    pub else_part: Option<Else>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Else {
    pub span: Span,
    pub kind: ElseKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElseKind {
    ElseIf(Box<IfPart>),
    Else(Body),
}

impl ElseKind {
    pub fn span(&self) -> Span {
        match self {
            ElseKind::Else(body) => body.span,
            ElseKind::ElseIf(if_) => if_.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub span: Span,
    pub cond: Expr,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Break {
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub span: Span,
    pub name: Ident,
    pub params: Vec<TypedIdent>,
    pub return_ty: Ty,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub span: Span,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Terminate {
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ty {
    pub span: Span,
    pub kind: TyKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TyKind {
    Name(Ident),
    Absent,
    Null,
    NoValue,
    Undefined,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Comparison(Comparison),
    Call(Call),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(lit) => lit.span,
            Expr::Comparison(comp) => comp.span,
            Expr::Call(call) => call.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub span: Span,
    pub kind: LiteralKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Absent,
    Null,
    NoValue,
    Undefined,
    String(String),
    Int(i64),
    Float(f64),
    True,
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub span: Span,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub kind: ComparisonKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonKind {
    NotEq,
    Eq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub span: Span,
    pub fn_name: Ident,
    pub args: CallArgs,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArgs {
    pub span: Span,
    pub args: Vec<CallArg>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    pub span: Span,
    pub expr: Expr,
    pub name: Ident,
}
