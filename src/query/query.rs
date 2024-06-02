use super::{
    expression::LogicalExpression,
    operator::{Operator, OperatorIterator},
    sink::Sink,
    window::{aggregation::Aggregation, window_descriptor::WindowDescriptor},
};

#[derive(Debug)]
pub struct QueryId(i32);

pub struct Query {
    operator: Operator,
    sink: Sink,
}

pub struct WindowedQueryBuilder {
    query_builder: QueryBuilder,
    key_fields: Option<Vec<String>>,
    descriptor: WindowDescriptor,
}

impl WindowedQueryBuilder {
    pub fn by_key(mut self, key: impl Into<String>) -> Self {
        if let Some(mut key_fields) = self.key_fields {
            key_fields.push(key.into());
        } else {
            self.key_fields = Some(vec![key.into()]);
        }
        self
    }

    pub fn apply(mut self, aggregation: impl Iterator<Item=Aggregation>) -> QueryBuilder {
        let child_operator = self.query_builder.operator;
        let aggregations = aggregation.collect();
        self.query_builder.operator = Operator::Window {
            child: Some(Box::new(child_operator)),
            descriptor: self.descriptor,
            aggregations,
            key_fields: self.key_fields,
        };
        self.query_builder
    }
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

    pub fn operators(&self) -> OperatorIterator {
        self.operator.iter()
    }

    pub fn sink(&self) -> &Sink {
        &self.sink
    }
}

impl QueryBuilder {
    pub fn from_source(source_name: String) -> Self {
        let operator = Operator::LogicalSource { source_name };
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

    pub fn window(self, descriptor: WindowDescriptor) -> WindowedQueryBuilder {
        WindowedQueryBuilder {
            query_builder: self,
            descriptor,
            key_fields: None,
        }
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
