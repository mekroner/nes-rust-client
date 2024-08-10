// this is for pretty printing of queries and expressions
use super::{
    expression::{
        binary_expression::{BinaryExpr, BinaryOp},
        expression::RawExpr,
        literal::Literal,
        unary_expression::{UnaryExpr, UnaryOp},
        Field,
    },
    join::Join,
    operator::{Filter, Map, Operator, Union, Window},
    sink::Sink,
    Query,
};

// TODO: Make stringify detail adjustable
pub enum StringifyDetail {
    Default,
}

pub fn stringify_query(query: &Query) -> String {
    format!(
        "{}.sink({});",
        stringify_operator(Some(query.operator())),
        stringify_sink(query.sink())
    )
}

fn stringify_operator(operator: Option<&Operator>) -> String {
    use Operator as O;
    match operator {
        Some(O::LogicalSource { source_name }) => format!("logical_source(\"{source_name}\")"),
        Some(O::Filter(filter)) => stringify_filter_operator(filter),
        Some(O::Map(map)) => stringify_map_operator(map),
        Some(O::Window(window)) => stringify_window_operator(window),
        Some(O::Join(join)) => stringify_join_operator(join),
        Some(O::Union(union)) => stringify_union_operator(union),
        None => String::new(),
    }
}

fn stringify_filter_operator(filter: &Filter) -> String {
    format!(
        "{}.filter({})",
        stringify_operator(filter.child.as_deref()),
        stringify_expr(&filter.expression.0)
    )
}

fn stringify_map_operator(map: &Map) -> String {
    format!(
        "{}.map({}, {})",
        stringify_operator(map.child.as_deref()),
        map.assigned_field,
        stringify_expr(&map.expression.0)
    )
}

fn stringify_window_operator(window: &Window) -> String {
    format!(
        "{}.window({})",
        stringify_operator(window.child.as_deref()),
        "TODO!!!"
    )
}

fn stringify_join_operator(join: &Join) -> String {
    format!(
        "{}.join({})",
        stringify_operator(join.child.as_deref()),
        "TODO!!!"
    )
}

fn stringify_union_operator(union: &Union) -> String {
    format!(
        "{}.union({})",
        stringify_operator(union.child.as_deref()),
        stringify_operator(Some(&union.operators)),
    )
}

pub fn stringify_expr(expr: &RawExpr) -> String {
    match expr {
        RawExpr::Literal(literal) => stringify_literal_expr(literal),
        RawExpr::Field(field) => stringify_field_expr(field),
        RawExpr::Unary(unary) => stringify_unary_expr(unary),
        RawExpr::Binary(binary) => stringify_binary_expr(binary),
    }
}

fn stringify_literal_expr(literal: &Literal) -> String {
    format!("{}", literal.value())
}

fn stringify_field_expr(field: &Field) -> String {
    format!("Field(\"{}\")", field.name())
}

fn stringify_unary_expr(expr: &UnaryExpr) -> String {
    let op = match expr.operator {
        UnaryOp::Negate => "!",
    };
    format!("{op}({})", stringify_expr(&expr.expr))
}

fn stringify_binary_expr(expr: &BinaryExpr) -> String {
    let op = match expr.operator {
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::Equals => "==",
        BinaryOp::Greater => ">",
        BinaryOp::GreaterEquals => ">=",
        BinaryOp::Less => "<",
        BinaryOp::LessEquals => "<=",
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Multiply => "*",
        BinaryOp::Divide => "/",
    };
    format!(
        "({} {op} {})",
        stringify_expr(&expr.lhs),
        stringify_expr(&expr.rhs)
    )
}

pub fn stringify_sink(sink: &Sink) -> String {
    match sink {
        Sink::NullOutput => format!("NullOutputSink"),
        Sink::Print => format!("PrintSink"),
        Sink::File {
            path,
            format,
            append,
        } => format!("FileSink(\"{path}\", {format}, {append})"),
    }
}
