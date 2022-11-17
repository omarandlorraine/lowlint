use lang_c::ast::Expression;
use lang_c::visit::Visit;
use lang_c::ast::InitDeclarator;
use lang_c::span::Span;
use lang_c::ast::BinaryOperatorExpression;
use lang_c::loc;

/// Checks that the variables have been initialized before their first use
pub struct UnsafeFnCall {
    pub problems: Vec<(Span, String)>,
}

impl Default for UnsafeFnCall {
    fn default() -> Self {
        Self {
            problems: vec![],
        }
    }
}

impl UnsafeFnCall {
    fn check_function(&mut self, span: &Span, fn_name: &String) {
        for t in vec!["atoi", "atof", "atol", "atoll"] {
            if fn_name == t {
                self.problems.push((*span, format!("call to {} is considered unsafe.", &fn_name)));
                self.problems.push((*span, format!("behavior of {} is undefined in case of overflow.", &fn_name)));
            }
        }

        if fn_name == "atoi" {
            self.problems.push((*span, format!("atoi does not detect or report errors; consider using strtol.")));
        }

        if fn_name == "gets" {
            self.problems.push((*span, format!("never use gets(). Its behavior is extremely dangerous.")));
        }
    }
}

impl<'ast> Visit<'ast> for UnsafeFnCall {
    fn visit_expression(
        &mut self,
        expression: &'ast lang_c::ast::Expression,
        span: &'ast Span
    ) {
        use lang_c::ast::Expression::Identifier;
        use lang_c::visit::visit_expression;
        match &expression {
            Expression::Call(call) => {
                match &call.node.callee.node {
                    Identifier(fn_name) => self.check_function(span, &fn_name.node.name),
                    _ => {}
                }
            }
            _ => {}
        }
        visit_expression(self, expression, span);
    }
}
