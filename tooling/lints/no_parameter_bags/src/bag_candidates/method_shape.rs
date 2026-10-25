use syn::{Expr, ExprStruct, ImplItemFn, Stmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MethodShape {
    Constructor,
    Accessor,
    Logic,
}

impl MethodShape {
    const SELF: &str = "Self";

    pub fn classify(function: &ImplItemFn, self_name: &str) -> MethodShape {
        match function.block.stmts.as_slice() {
            [Stmt::Expr(expr, None)] => Self::of_expression(expr, self_name),
            _ => MethodShape::Logic,
        }
    }

    fn of_expression(expr: &Expr, self_name: &str) -> MethodShape {
        match expr {
            Expr::Field(field) if matches!(&*field.base, Expr::Path(base) if base.path.is_ident("self")) => {
                MethodShape::Accessor
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
