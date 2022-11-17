use lang_c::ast::Expression;
use lang_c::visit::Visit;
use lang_c::span::Span;

/// Checks that the variables have been initialized before their first use
#[derive(Default)]
pub struct UnsafeFnCall {
    pub problems: Vec<(Span, String)>,
}

impl UnsafeFnCall {
    fn check_function(&mut self, span: &Span, fn_name: &String) {
        for t in &["atoi", "atof", "atol", "atoll"] {
            if fn_name == t {
                self.problems.push((*span, format!("call to {} is considered unsafe.", &fn_name)));
                self.problems.push((*span, format!("behavior of {} is undefined in case of overflow.", &fn_name)));
            }
        }

        if fn_name == "atoi" {
            self.problems.push((*span, "atoi does not detect or report errors; consider using strtol.".to_string()));
        }

        if fn_name == "gets" {
            self.problems.push((*span, "never use gets(). Its behavior is extremely dangerous.".to_string()));
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
        if let Expression::Call(call) = &expression {
            if let Identifier(fn_name) = &call.node.callee.node {
                self.check_function(span, &fn_name.node.name)
            }
        }
        visit_expression(self, expression, span);
    }
}
