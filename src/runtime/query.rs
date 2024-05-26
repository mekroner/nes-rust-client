use crate::operator::{LogicalExpression, Operator, Sink, WindowDescriptor};

#[derive(Debug)]
pub struct QueryId(i32);

pub struct Query {
    operator: Operator,
    sink: Sink,
}

// FIXME: I should use a QueryBuilder.
// The QueryBuilder::sink() should be the last element in the chain and build the Query.
// Each operator in the chain should return Result<QueryBuilder, QueryBuildError>
pub struct QueryBuilder {
    operator: Operator,
}

impl Query {
    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn sink(&self) -> &Sink {
        &self.sink
    }
}
impl QueryBuilder {
    pub fn from_source(source_name: String) -> Self {
        let operator = Operator::LogicalSource {
            source_name,
            child: None,
        };
        QueryBuilder { operator }
    }

    // NOTE: This violates single responsibility principle. Maybe we should use a dedicated build
    // function
    pub fn sink(self, sink: Sink) -> Query {
        Query {
            sink,
            operator: self.operator,
        }
    }

    pub fn filter(mut self, expression: LogicalExpression) -> Self {
        let child_operator = self.operator;
        self.operator = Operator::Filter {
            child: Some(Box::new(child_operator)),
            expression,
        };
        self
    }

    pub fn window(mut self, descriptor: WindowDescriptor) -> Self {
        let child_operator = self.operator;
        self.operator = Operator::Window {
            child: Some(Box::new(child_operator)),
            descriptor,
        };
        self
    }

    pub fn project(self) -> Self {
        unimplemented!();
    }

    pub fn map(self) -> Self {
        unimplemented!();
    }

    pub fn flat_map(self) -> Self {
        unimplemented!();
    }

    pub fn join_with(self) -> Self {
        unimplemented!();
    }
}

#[cfg(test)]
mod query_tests {
    use crate::{
        operator::{LogicalExpression, Sink},
        runtime::query::QueryBuilder,
    };

    #[test]
    fn query_test0() {
        use LogicalExpression as E;
        let query = QueryBuilder::from_source("default".to_string())
            .filter(E::Equal(
                Box::new(E::Attribute("value".to_string())),
                Box::new(E::Literal(0)),
            ))
            .sink(Sink::NullOutput);
        dbg!(query.operator);
    }
}
