use lang_c::visit::Visit;
use lang_c::ast::InitDeclarator;
use lang_c::ast::CallExpression;
use lang_c::span::Span;
use lang_c::ast::BinaryOperatorExpression;
use lang_c::loc;

/// Checks that the variables have been initialized before their first use
pub struct VarInitCheck {
    pub problems: Vec<(Span, String)>,
    uninit: Vec<String>
}

impl Default for VarInitCheck {
    fn default() -> Self {
        Self {
            problems: vec![],
            uninit: vec![],
        }
    }
}

impl<'ast> Visit<'ast> for VarInitCheck {
    fn visit_init_declarator(
        &mut self,
        init_declarator: &'ast InitDeclarator,
        span: &'ast Span
    ) {
        use lang_c::ast::DeclaratorKind::Identifier;
        use lang_c::visit::visit_init_declarator;
        if init_declarator.initializer.is_none() {
            match &init_declarator.declarator.node.kind.node {
                Identifier(node) => {
                    self.uninit.push(node.node.name.clone());
                }
                _ => {}
            }
        }
        visit_init_declarator(self, init_declarator, span);
    }

    fn visit_binary_operator_expression(
        &mut self,
        binary_operator_expression: &'ast BinaryOperatorExpression,
        span: &'ast Span
    ) {
        use lang_c::visit::visit_binary_operator_expression;
        use lang_c::ast::Expression::Identifier;
        match &binary_operator_expression.lhs.node {
            Identifier(identifier) => {
                self.uninit.retain(|n| n != &identifier.node.name);
            }
            _ => {}
        }
        visit_binary_operator_expression(self, binary_operator_expression, span);
    }

    fn visit_expression(
        &mut self,
        expression: &'ast lang_c::ast::Expression,
        span: &'ast Span
    ) {
        use lang_c::ast::Expression::Identifier;
        use lang_c::visit::visit_expression;
        match &expression {
            Identifier(identifier) => {
                self.problems.push((*span, format!("{:?} Use before initialization: {}", &span, &identifier.node.name)));
            }
            _ => {
            }
        }
        visit_expression(self, expression, span);
    }

    fn visit_call_expression(
        &mut self,
        call_expression: &'ast CallExpression,
        _span: &'ast Span,
    ) {
        use lang_c::visit::visit_expression;
        for argument in &call_expression.arguments {
            visit_expression(self, &argument.node, &argument.span);
        }
    }
}
