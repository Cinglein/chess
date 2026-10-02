use syn::{Expr, ExprStruct, ImplItemFn, Stmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MethodShape {
    Constructor,
    Accessor,
    Updater,
    Logic,
}

impl MethodShape {
    const SELF: &str = "Self";

    pub fn classify(function: &ImplItemFn, self_name: &str) -> MethodShape {
        let [Stmt::Expr(expr, None)] = function.block.stmts.as_slice() else {
            return MethodShape::Logic;
        };
        match expr {
            Expr::Field(field) if matches!(&*field.base, Expr::Path(base) if base.path.is_ident("self")) => {
                MethodShape::Accessor
            }
            Expr::Struct(literal) if Self::builds(literal, self_name) && literal.rest.is_some() => {
                MethodShape::Updater
            }
            Expr::Struct(literal) if Self::builds(literal, self_name) => MethodShape::Constructor,
            _ => MethodShape::Logic,
        }
    }

    pub fn is_trivial(self) -> bool {
        self != MethodShape::Logic
    }

    fn builds(literal: &ExprStruct, self_name: &str) -> bool {
        literal
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == Self::SELF || segment.ident == self_name)
    }
}
