use syn::{Block, Expr, FnArg, ImplItemFn, ReturnType, Stmt, Type};

use super::declarations::Declarations;
use super::impl_context::ImplContext;
use super::type_name::TypeName;

pub struct FunctionShape {
    site: String,
    target: Option<TypeName>,
    tuple_holds_vertex: bool,
    vertex_parameters: Vec<TypeName>,
    projects_field: bool,
}

impl FunctionShape {
    pub fn of(
        path: &str,
        function: &ImplItemFn,
        context: &ImplContext,
        declarations: &Declarations,
    ) -> FunctionShape {
        let name = &function.sig.ident;
        let resolve =
            |ty: &Type| declarations.resolve(TypeName::of(ty).or_self(context.self_type()));
        let returned = match &function.sig.output {
            ReturnType::Type(_, ty) => Some(&**ty),
            ReturnType::Default => None,
        };
        let target = returned.and_then(|ty| match ty {
            Type::Reference(_) | Type::Tuple(_) => None,
            other => {
                let returned = resolve(other);
                declarations.is_vertex(&returned).then_some(returned)
            }
        });
        FunctionShape {
            site: format!("{path}:{}: fn {name}", name.span().start().line),
            target,
            tuple_holds_vertex: returned.is_some_and(|ty| {
                matches!(
                    ty,
                    Type::Tuple(tuple)
                        if tuple.elems.iter().any(|element| declarations.is_vertex(&resolve(element)))
                )
            }),
            vertex_parameters: function
                .sig
                .inputs
                .iter()
                .filter_map(|argument| match argument {
                    FnArg::Typed(typed) => Some(resolve(&typed.ty)),
                    FnArg::Receiver(_) => None,
                })
                .filter(|parameter| declarations.is_vertex(parameter))
                .collect(),
            projects_field: Self::is_field_projection(&function.block),
        }
    }

    pub fn site(&self) -> &str {
        &self.site
    }

    pub fn target(&self) -> Option<&TypeName> {
        self.target.as_ref()
    }

    pub fn tuple_holds_vertex(&self) -> bool {
        self.tuple_holds_vertex
    }

    pub fn vertex_parameters(&self) -> &[TypeName] {
        &self.vertex_parameters
    }

    pub fn projects_field(&self) -> bool {
        self.projects_field
    }

    fn is_field_projection(block: &Block) -> bool {
        matches!(
            block.stmts.as_slice(),
            [Stmt::Expr(Expr::Field(field), None)]
                if matches!(&*field.base, Expr::Path(base) if base.path.is_ident("self"))
        )
    }
}
