use syn::{Expr, ImplItemFn, Stmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MethodShape {
    Constructor,
    Accessor,
    Updater,
    Logic,
}

impl MethodShape {
    pub fn classify(function: &ImplItemFn) -> MethodShape {
        let [Stmt::Expr(expr, None)] = function.block.stmts.as_slice() else {
            return MethodShape::Logic;
        };
        match expr {
            Expr::Field(field) if matches!(&*field.base, Expr::Path(base) if base.path.is_ident("self")) => {
                MethodShape::Accessor
            }
            Expr::Struct(literal) if literal.rest.is_some() => MethodShape::Updater,
            Expr::Struct(_) => MethodShape::Constructor,
            _ => MethodShape::Logic,
        }
    }

    pub fn is_trivial(self) -> bool {
        self != MethodShape::Logic
    }
}
