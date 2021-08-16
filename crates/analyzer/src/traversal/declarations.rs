use crate::context::AnalyzerContext;
use crate::errors::{AlreadyDefined, FatalError};
use crate::namespace::scopes::BlockScope;
use crate::namespace::types::FixedSize;
use crate::traversal::{expressions, types};
use fe_common::diagnostics::Label;
use fe_common::utils::humanize::pluralize_conditionally;
use fe_parser::ast as fe;
use fe_parser::node::Node;
use std::convert::TryFrom;

/// Gather context information for var declarations and check for type errors.
pub fn var_decl(scope: &mut BlockScope, stmt: &Node<fe::FuncStmt>) -> Result<(), FatalError> {
    if let fe::FuncStmt::VarDecl { target, typ, value } = &stmt.kind {
        let declared_type = match FixedSize::try_from(types::type_desc(scope, typ)?) {
            Ok(typ) => typ,
            Err(_) => {
                // If this conversion fails, the type must be a map (for now at least)
                scope.error(
                    "invalid variable type",
                    typ.span,
                    "`Map` type can only be used as a contract field",
                );
                return Err(FatalError::new());
            }
        };

        if let Some(value) = value {
            let value_attributes =
                expressions::assignable_expr(scope, value, Some(&declared_type.clone().into()))?;

            if declared_type != value_attributes.typ {
                scope.type_error(
                    "type mismatch",
                    value.span,
                    &declared_type,
                    &value_attributes.typ,
                );
            }
        }

        scope.root.add_declaration(typ, declared_type.clone());
        add_var(scope, target, declared_type)?;
        return Ok(());
    }

    unreachable!()
}

/// Add declared variables to the scope.
fn add_var(
    scope: &mut BlockScope,
    target: &Node<fe::VarDeclTarget>,
    typ: FixedSize,
) -> Result<(), FatalError> {
    match (&target.kind, typ) {
        (fe::VarDeclTarget::Name(name), typ) => {
            if let Err(AlreadyDefined(prev_span)) = scope.add_var(name, typ, target.span) {
                scope.fancy_error(
                    "duplicate variable definition",
                    vec![
                        Label::primary(prev_span, &format!("`{}` first defined here", name)),
                        Label::secondary(target.span, &format!("`{}` redefined here", name)),
                    ],
                    vec![],
                )
            }
            Ok(())
        }
        (fe::VarDeclTarget::Tuple(items), FixedSize::Tuple(items_ty)) => {
            let items_ty = items_ty.items;
            let items_ty_len = items_ty.as_vec().len();
            if items.len() != items_ty_len {
                scope.fancy_error(
                    "invalid declaration",
                    vec![Label::primary(target.span, "")],
                    vec![format!(
                        "Tuple declaration has {} {} but the specified tuple type has {} {}",
                        items.len(),
                        pluralize_conditionally("item", items.len()),
                        items_ty_len,
                        pluralize_conditionally("item", items_ty_len),
                    )],
                );
                return Err(FatalError::new());
            }
            for (item, item_ty) in items.iter().zip(items_ty.into_iter()) {
                add_var(scope, item, item_ty)?;
            }
            Ok(())
        }
        (fe::VarDeclTarget::Tuple(_), typ) if !matches!(typ, FixedSize::Tuple(_)) => {
            scope.fancy_error("invalid declaration",
                                vec![Label::primary(target.span, "")],
                                vec![format!("Tuple declaration targets need to be declared with the tuple type but here the type is {}", typ)]);
            Err(FatalError::new())
        }
        _ => Err(FatalError::new()),
    }
}