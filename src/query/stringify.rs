// this is for pretty printing of queries and expressions

use super::{
    join::Join,
    operator::{Filter, Map, Operator, Projection, Union, Window},
    sink::Sink,
    time::{Duration, TimeCharacteristic, TimeUnit},
    window::{aggregation::Aggregation, window_descriptor::WindowDescriptor},
    Query,
};
use crate::expression::{
    binary_expression::{BinaryExpr, BinaryOp},
    expression::RawExpr,
    literal::Literal,
    unary_expression::{UnaryExpr, UnaryOp},
    Field,
};

// TODO: Make stringify detail adjustable
pub enum StringifyDetail {
    Default,
}

/// Allows to pretty print a `Query`.
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
        Some(O::Projection(projection)) => stringify_projection_operator(projection),
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

fn stringify_projection_operator(projection: &Projection) -> String {
    let mut fields = String::new();
    for field in &projection.fields {
        let field_name = field.name();
        if let Some(as_name) = field.projected_name() {
            fields.push_str(&format!("Field(\"{field_name}\").as(\"{as_name}\"), "));
            continue;
        }
        fields.push_str(&format!("Field(\"{field_name}\"), "));
    }
    format!(
        "{}.project({fields})",
        stringify_operator(projection.child.as_deref()),
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

// FIXME: Window
fn stringify_window_operator(window: &Window) -> String {
    format!(
        "{}.window({}).apply({})",
        stringify_operator(window.child.as_deref()),
        stringify_window_descriptor(&window.descriptor),
        stringify_aggregations(&window.aggregations),
    )
}

fn stringify_window_descriptor(desr: &WindowDescriptor) -> String {
    match desr {
        WindowDescriptor::TumblingWindow {
            duration,
            time_character,
        } => format!(
            "TumblingWindow({}, {})",
            stringify_duration(duration),
            stringify_time_character(time_character)
        ),
    }
}

fn stringify_duration(duration: &Duration) -> String {
    format!("{}{}", duration.amount, stringify_time_unit_symbol(&duration.unit))
}

fn stringify_time_unit_symbol(unit: &TimeUnit) -> String {
    match unit {
        TimeUnit::Milliseconds => "ms",
        TimeUnit::Seconds => "s",
        TimeUnit::Minutes => "min",
        TimeUnit::Hours => "h",
        TimeUnit::Days => "d",
    }.to_string()
}

fn stringify_time_unit(unit: &TimeUnit) -> String {
    match unit {
        TimeUnit::Milliseconds => "Milliseconds",
        TimeUnit::Seconds => "Seconds",
        TimeUnit::Minutes => "Minutes",
        TimeUnit::Hours => "Hours",
        TimeUnit::Days => "Days",
    }.to_string()
}

fn stringify_time_character(character: &TimeCharacteristic) -> String {
    match character {
        TimeCharacteristic::EventTime { field_name, unit } => {
            format!("EventTime(\"{field_name}\", {})", stringify_time_unit(unit))
        }
    }
}

fn stringify_aggregations(aggregations: &[Aggregation]) -> String {
    use crate::query::window::aggregation::AggregationType;
    let mut result = String::new();
    for agg in aggregations {
        let field = agg.field();
        let projected_field = agg.projected_field();
        let agg_type = agg.agg_type();
        let agg_type_str = match agg_type {
            AggregationType::Sum => "sum",
            AggregationType::Average => "average",
            AggregationType::Min => "min",
            AggregationType::Max => "max",
            AggregationType::Median => "median",
            AggregationType::Count => "count",
        };
        let agg_str = match (field, projected_field) {
            (None, None) => format!("{agg_type_str}(), "),
            (None, Some(p)) => format!("{agg_type_str}(as(\"{}\")), ", p.name()),
            (Some(f), None) => format!("{agg_type_str}(Field(\"{}\")), ", f.name()),
            (Some(f), Some(p)) => format!(
                "{agg_type_str}(Field(\"{}\").as(\"{}\")), ",
                f.name(),
                p.name()
            ),
        };
        result.push_str(&agg_str);
    }
    result
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
        UnaryOp::Absolute => "abs",
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
        BinaryOp::Remainder => "%",
        BinaryOp::Power => "^"
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
