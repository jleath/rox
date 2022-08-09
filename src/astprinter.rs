pub struct AstPrinter {}

use crate::expr::{Expr, ExprVisitor};
use crate::token::{Literal, Token};

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }
    pub fn print(&mut self, expr: Expr) -> String {
        self.evaluate(expr)
    }

    fn parenthesize(&mut self, name: String, exprs: &[Expr]) -> String {
        let mut expr_str = format!("({}", name);
        for expr in exprs {
            expr_str.push(' ');
            let result = self.evaluate(expr.clone());
            expr_str.push_str(result.as_str());
        }
        expr_str.push(')');
        expr_str
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: Box<Expr>, op: Token, right: Box<Expr>) -> String {
        let left = *left;
        let right = *right;
        self.parenthesize(op.lexeme(), &[left, right])
    }

    fn visit_grouping_expr(&mut self, expr: Box<Expr>) -> String {
        let expr = *expr;
        self.parenthesize(String::from("group"), &[expr])
    }

    fn visit_literal_expr(&mut self, value: Literal) -> String {
        if let Literal::Nil = value {
            return String::from("nil");
        }
        match value {
            Literal::Nil => String::from("nil"),
            Literal::Number(value) => format!("{}", value),
            Literal::Boolean(value) => format!("{}", value),
            Literal::LoxString(value) => value,
        }
    }

    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> String {
        let right = *right;
        self.parenthesize(operator.lexeme(), &[right])
    }
}
