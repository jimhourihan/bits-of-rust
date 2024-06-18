/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::calculator::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Number = String;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.into()
}
pub type Symbol = String;
pub fn symbol(_ctx: &Ctx, token: Token) -> Symbol {
    token.value.into()
}
pub type Expr = SequenceExpr;
pub fn expr_sequence_expr(_ctx: &Ctx, sequence_expr: SequenceExpr) -> Expr {
    sequence_expr
}
pub type SequenceExpr = TupleExpr1;
pub fn sequence_expr_tuple_expr1(_ctx: &Ctx, tuple_expr1: TupleExpr1) -> SequenceExpr {
    tuple_expr1
}
pub type TupleExpr1 = Vec<TupleExpr>;
pub fn tuple_expr1_c1(
    _ctx: &Ctx,
    mut tuple_expr1: TupleExpr1,
    tuple_expr: TupleExpr,
) -> TupleExpr1 {
    tuple_expr1.push(tuple_expr);
    tuple_expr1
}
pub fn tuple_expr1_tuple_expr(_ctx: &Ctx, tuple_expr: TupleExpr) -> TupleExpr1 {
    vec![tuple_expr]
}
pub type TupleExpr = ConditionalExpr1;
pub fn tuple_expr_conditional_expr1(
    _ctx: &Ctx,
    conditional_expr1: ConditionalExpr1,
) -> TupleExpr {
    conditional_expr1
}
pub type ConditionalExpr1 = Vec<ConditionalExpr>;
pub fn conditional_expr1_c1(
    _ctx: &Ctx,
    mut conditional_expr1: ConditionalExpr1,
    conditional_expr: ConditionalExpr,
) -> ConditionalExpr1 {
    conditional_expr1.push(conditional_expr);
    conditional_expr1
}
pub fn conditional_expr1_conditional_expr(
    _ctx: &Ctx,
    conditional_expr: ConditionalExpr,
) -> ConditionalExpr1 {
    vec![conditional_expr]
}
#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    MatchAny,
}
pub fn pattern_literal(_ctx: &Ctx, literal: Literal) -> Pattern {
    Pattern::Literal(literal)
}
pub fn pattern_match_any(_ctx: &Ctx) -> Pattern {
    Pattern::MatchAny
}
#[derive(Debug, Clone)]
pub struct CaseMatch {
    pub pattern: Pattern,
    pub binary_expr: BinaryExpr,
}
pub fn case_match_c1(
    _ctx: &Ctx,
    pattern: Pattern,
    binary_expr: BinaryExpr,
) -> CaseMatch {
    CaseMatch { pattern, binary_expr }
}
pub type CaseArms = CaseMatch1;
pub fn case_arms_case_match1(_ctx: &Ctx, case_match1: CaseMatch1) -> CaseArms {
    case_match1
}
pub type CaseMatch1 = Vec<CaseMatch>;
pub fn case_match1_c1(
    _ctx: &Ctx,
    mut case_match1: CaseMatch1,
    case_match: CaseMatch,
) -> CaseMatch1 {
    case_match1.push(case_match);
    case_match1
}
pub fn case_match1_case_match(_ctx: &Ctx, case_match: CaseMatch) -> CaseMatch1 {
    vec![case_match]
}
#[derive(Debug, Clone)]
pub struct IfThenElseExpr {
    pub binary_expr_2: BinaryExpr,
    pub binary_expr_4: BinaryExpr,
    pub binary_expr_6: BinaryExpr,
}
#[derive(Debug, Clone)]
pub struct ConditionalExprC3 {
    pub binary_expr: BinaryExpr,
    pub case_arms: CaseArms,
}
#[derive(Debug, Clone)]
pub enum ConditionalExpr {
    BinaryExpr(BinaryExpr),
    IfThenElseExpr(IfThenElseExpr),
    C3(ConditionalExprC3),
}
pub fn conditional_expr_binary_expr(
    _ctx: &Ctx,
    binary_expr: BinaryExpr,
) -> ConditionalExpr {
    ConditionalExpr::BinaryExpr(binary_expr)
}
pub fn conditional_expr_if_then_else_expr(
    _ctx: &Ctx,
    binary_expr_2: BinaryExpr,
    binary_expr_4: BinaryExpr,
    binary_expr_6: BinaryExpr,
) -> ConditionalExpr {
    ConditionalExpr::IfThenElseExpr(IfThenElseExpr {
        binary_expr_2,
        binary_expr_4,
        binary_expr_6,
    })
}
pub fn conditional_expr_c3(
    _ctx: &Ctx,
    binary_expr: BinaryExpr,
    case_arms: CaseArms,
) -> ConditionalExpr {
    ConditionalExpr::C3(ConditionalExprC3 {
        binary_expr,
        case_arms,
    })
}
#[derive(Debug, Clone)]
pub struct LogicalOrExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct LogicalAndExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct EqExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct NotEqExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct LTExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct GTExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct LTEqExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct GTEqExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct AddExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct SubExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct MultExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct DivExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct ModExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub struct ExpExpr {
    pub binary_expr_1: Box<BinaryExpr>,
    pub binary_expr_3: Box<BinaryExpr>,
}
#[derive(Debug, Clone)]
pub enum BinaryExpr {
    UnaryExpr(UnaryExpr),
    LogicalOrExpr(LogicalOrExpr),
    LogicalAndExpr(LogicalAndExpr),
    EqExpr(EqExpr),
    NotEqExpr(NotEqExpr),
    LTExpr(LTExpr),
    GTExpr(GTExpr),
    LTEqExpr(LTEqExpr),
    GTEqExpr(GTEqExpr),
    AddExpr(AddExpr),
    SubExpr(SubExpr),
    MultExpr(MultExpr),
    DivExpr(DivExpr),
    ModExpr(ModExpr),
    ExpExpr(ExpExpr),
}
pub fn binary_expr_unary_expr(_ctx: &Ctx, unary_expr: UnaryExpr) -> BinaryExpr {
    BinaryExpr::UnaryExpr(unary_expr)
}
pub fn binary_expr_logical_or_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::LogicalOrExpr(LogicalOrExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_logical_and_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::LogicalAndExpr(LogicalAndExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_eq_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::EqExpr(EqExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_not_eq_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::NotEqExpr(NotEqExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_ltexpr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::LTExpr(LTExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_gtexpr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::GTExpr(GTExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_lteq_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::LTEqExpr(LTEqExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_gteq_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::GTEqExpr(GTEqExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_add_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::AddExpr(AddExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_sub_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::SubExpr(SubExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_mult_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::MultExpr(MultExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_div_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::DivExpr(DivExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_mod_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::ModExpr(ModExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
pub fn binary_expr_exp_expr(
    _ctx: &Ctx,
    binary_expr_1: BinaryExpr,
    binary_expr_3: BinaryExpr,
) -> BinaryExpr {
    BinaryExpr::ExpExpr(ExpExpr {
        binary_expr_1: Box::new(binary_expr_1),
        binary_expr_3: Box::new(binary_expr_3),
    })
}
#[derive(Debug, Clone)]
pub enum UnaryExpr {
    Literal(Literal),
    TupleExpr(Box<TupleExpr>),
    SequenceExpr(Box<SequenceExpr>),
    UminusExpr(Box<UnaryExpr>),
    UplusExpr(Box<UnaryExpr>),
    NotExpr(Box<UnaryExpr>),
}
pub fn unary_expr_literal(_ctx: &Ctx, literal: Literal) -> UnaryExpr {
    UnaryExpr::Literal(literal)
}
pub fn unary_expr_tuple_expr(_ctx: &Ctx, tuple_expr: TupleExpr) -> UnaryExpr {
    UnaryExpr::TupleExpr(Box::new(tuple_expr))
}
pub fn unary_expr_sequence_expr(_ctx: &Ctx, sequence_expr: SequenceExpr) -> UnaryExpr {
    UnaryExpr::SequenceExpr(Box::new(sequence_expr))
}
pub fn unary_expr_uminus_expr(_ctx: &Ctx, unary_expr: UnaryExpr) -> UnaryExpr {
    UnaryExpr::UminusExpr(Box::new(unary_expr))
}
pub fn unary_expr_uplus_expr(_ctx: &Ctx, unary_expr: UnaryExpr) -> UnaryExpr {
    UnaryExpr::UplusExpr(Box::new(unary_expr))
}
pub fn unary_expr_not_expr(_ctx: &Ctx, unary_expr: UnaryExpr) -> UnaryExpr {
    UnaryExpr::NotExpr(Box::new(unary_expr))
}
#[derive(Debug, Clone)]
pub enum Literal {
    Number(Number),
    Symbol(Symbol),
    True,
    False,
}
pub fn literal_number(_ctx: &Ctx, number: Number) -> Literal {
    Literal::Number(number)
}
pub fn literal_symbol(_ctx: &Ctx, symbol: Symbol) -> Literal {
    Literal::Symbol(symbol)
}
pub fn literal_true(_ctx: &Ctx) -> Literal {
    Literal::True
}
pub fn literal_false(_ctx: &Ctx) -> Literal {
    Literal::False
}
