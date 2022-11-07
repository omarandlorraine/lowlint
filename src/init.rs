use lang_c::visit::Visit;
use lang_c::ast::InitDeclarator;
use lang_c::span::Span;
use lang_c::ast::DeclaratorKind::Identifier;

/// Checks that the variables have been initialized before their first use
pub struct VarInitCheck {
    uninit: Vec<String>
}

impl Default for VarInitCheck {
    fn default() -> Self {
        Self {
            uninit: vec![]
        }
    }
}

impl<'ast> Visit<'ast> for VarInitCheck {
    fn visit_init_declarator(
        &mut self,
        init_declarator: &'ast InitDeclarator,
        span: &'ast Span
    ) {
        if init_declarator.initializer.is_none() {
            match &init_declarator.declarator.node.kind.node {
                Identifier(node) => {
                    println!("{}", node.node.name.clone());
                    self.uninit.push(node.node.name.clone());
                }
                _ => {}
            }
        }
    }
}
