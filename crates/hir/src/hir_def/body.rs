use cranelift_entity::{PrimaryMap, SecondaryMap};
use fe_parser2::ast::{self, Stmt};

use crate::span::HirOrigin;

use super::{Expr, ExprId, Pat, PatId, StmtId};

#[salsa::tracked]
pub struct Body {
    #[id]
    pub kind: BodyKind,

    pub stmts: PrimaryMap<StmtId, Stmt>,
    pub exprs: PrimaryMap<ExprId, Expr>,
    pub pats: PrimaryMap<PatId, Pat>,

    pub(crate) stmt_source_map: SecondaryMap<StmtId, HirOrigin<ast::AstPtr<ast::Stmt>>>,
    pub(crate) expr_source_map: SecondaryMap<ExprId, HirOrigin<ast::AstPtr<ast::Expr>>>,
    pub(crate) pat_source_map: SecondaryMap<ExprId, HirOrigin<ast::AstPtr<ast::Expr>>>,
    pub(crate) ast: HirOrigin<ast::AstPtr<ast::Expr>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyKind {
    /// This is a body appearing in a item, e.g., a function or const item.
    DefBlock(super::ItemKind),
    /// This is a body appearing in array types or
    NamelessConst,

    /// The body is invalid.
    /// This is used to represent bodies that failed to parse.
    Invalid,
}