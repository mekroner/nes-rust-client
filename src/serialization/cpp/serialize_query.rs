use crate::{
    operator::{
        Duration, LogicalExpression, Operator, Sink, TimeCharacteristic, TimeUnit, WindowDescriptor,
    },
    runtime::query::Query,
};

pub fn serialize(query: Query) -> String {
    format!(
        "Query::{}.sink({});",
        serialize_operator(Some(query.operator())),
        serialize_sink(query.sink())
    )
}

fn serialize_operator(operator: Option<&Operator>) -> String {
    use Operator as O;
    match operator {
        Some(O::LogicalSource { source_name, child }) => {
            format!("from(\"{}\")", source_name,)
        }
        Some(O::Window { descriptor, child }) => format!(
            "{}.window({})",
            serialize_operator(child.as_deref()),
            serialize_window_descriptor(descriptor)
        ),
        Some(O::Filter { expression, child }) => format!(
            "{}.filter({})",
            serialize_operator(child.as_deref()),
            serialize_logical_expression(expression)
        ),
        None => "".to_string(),
    }
}

fn serialize_logical_expression(expression: &LogicalExpression) -> String {
    use LogicalExpression as LE;
    match expression {
        LE::Attribute(attr) => format!("Attribute(\"{}\")", attr),
        LE::Literal(lit) => format!("{}", lit),
        LE::Equal(rh, lh) => format!(
            "{} == {}",
            serialize_logical_expression(rh),
            serialize_logical_expression(lh)
        ),
        LE::NotEqual(_, _) => todo!(),
        LE::And(_, _) => todo!(),
        LE::Or(_, _) => todo!(),
    }
}

fn serialize_window_descriptor(descriptor: &WindowDescriptor) -> String {
    use WindowDescriptor as WD;
    match descriptor {
        WD::TumblingWindow {
            duration,
            time_character,
        } => format!(
            "TumblingWindow::of({}, {})",
            serialize_time_character(time_character),
            serialize_duration(duration)
        ),
    }
}

fn serialize_time_character(time_character: &TimeCharacteristic) -> String {
    match time_character {
        TimeCharacteristic::EventTime { field_name, unit } => format!(
            "EventTime(Attribute(\"{}\"), {})",
            field_name,
            serialize_time_unit(*unit)
        ),
    }
}

fn serialize_duration(duration: &Duration) -> String {
    format!("{}({})", duration.unit().to_string(), duration.amount())
}

fn serialize_time_unit(unit: TimeUnit) -> String {
    format!("{}()", unit.to_string())
}

// TODO: Implement function
fn serialize_sink(sink: &Sink) -> String {
    match sink {
        Sink::NullOutput => format!("NullOutputSinkDescriptor::create()"),
        Sink::Print => format!("PrintSinkDescriptor::create()"),
        Sink::File {
            path,
            format,
            append,
        } => todo!(),
    }
}

#[cfg(test)]
mod serialize_query_test {
    use crate::{
        operator::{
            Duration, LogicalExpression, Sink, TimeCharacteristic, TimeUnit, WindowDescriptor,
        },
        runtime::query::QueryBuilder,
        serialization::cpp::serialize_query::serialize,
    };

    #[test]
    fn test_serialize_filter() {
        use LogicalExpression as E;
        let expected = "Query::from(\"default\")\
            .filter(Attribute(\"value\") == 0)\
            .sink(NullOutputSinkDescriptor::create());";

        let query = QueryBuilder::from_source("default".to_string())
            .filter(E::Equal(
                Box::new(E::Attribute("value".to_string())),
                Box::new(E::Literal(0)),
            ))
            .sink(Sink::NullOutput);

        let serialized_query = serialize(query);
        assert_eq!(expected, serialized_query);
    }

    #[test]
    fn test_serialize_window() {
        let expected = "Query::from(\"input1\")\
            .window(TumblingWindow::of(EventTime(Attribute(\"timestamp\"), Milliseconds()), Seconds(10)))\
            .sink(NullOutputSinkDescriptor::create());";

        let query = QueryBuilder::from_source("input1".to_string())
            .window(WindowDescriptor::TumblingWindow {
                duration: Duration::seconds(10),
                time_character: TimeCharacteristic::EventTime {
                    unit: TimeUnit::Milliseconds,
                    field_name: "timestamp".to_string(),
                },
            })
            .sink(Sink::NullOutput);

        let serialized_query = serialize(query);
        assert_eq!(expected, serialized_query);
    }

    #[test]
    fn test_serialize_map() {
        use LogicalExpression as E;
        let expected = "Query::from(\"input1\")\
            .map(Attribute(\"value\") = Attribute(\"value\") * 2)\
            .sink(NullOutputSinkDescriptor::create());";

        let query = QueryBuilder::from_source("default".to_string())
            .filter(E::Equal(
                Box::new(E::Attribute("value".to_string())),
                Box::new(E::Literal(0)),
            ))
            .sink(Sink::NullOutput);

        let serialized_query = serialize(query);
        assert_eq!(expected, serialized_query);
    }

    #[test]
    fn test() {
        let expected = "Query::from(\"wind_turbines\")\
            .window(TumblingWindow::of(EventTime(Attribute(\"features_properties_updated\")), Minutes(10)))\
            .byKey(Attribute(\"metadata_id\"))\
            .apply(Sum(Attribute(\"features_properties_mag\")))\
            .sink(FileSinkDescriptor::create(\"result.csv\", \"CSV_FORMAT\", \"APPEND\"));";

        let query = QueryBuilder::from_source("wind_turbines".to_string()).sink(Sink::File {
            path: "result.csv".to_string(),
            format: "CSV_FORMAT".to_string(),
            append: true,
        });

        let serialized_query = serialize(query);
        assert_eq!(expected, serialized_query);
    }
}
