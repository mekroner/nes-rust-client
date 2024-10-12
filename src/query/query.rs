use serde::{Deserialize, Serialize};

use super::{
    join::JoinWhereBuilder,
    operator::{Filter, Map, Operator, OperatorIterator, Projection, Union, Window},
    sink::Sink,
    window::{aggregation::Aggregation, window_descriptor::WindowDescriptor},
};
use crate::expression::{ArithmeticExpr, Field, LogicalExpr};

/// A `Query` object is user code API to specify a NES query. Queries are used to manipulate stream
/// contents. To create a query use the `QueryBuilder` API.
/// A `Query` consists of a tree of `Operators` and a single `Sink`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub(super) operator: Operator,
    pub(super) sink: Sink,
}

/// The `QueryBuilder` is used to create `Query` objects.
#[derive(Debug, Clone)]
pub struct WindowedQueryBuilder {
    query_builder: QueryBuilder,
    key_fields: Option<Vec<String>>,
    descriptor: WindowDescriptor,
}

impl WindowedQueryBuilder {
    pub fn by_key(mut self, key: impl Into<String>) -> Self {
        if let Some(ref mut key_fields) = self.key_fields {
            key_fields.push(key.into());
        } else {
            self.key_fields = Some(vec![key.into()]);
        }
        self
    }

    pub fn apply(mut self, aggregation: impl IntoIterator<Item = Aggregation>) -> QueryBuilder {
        let child_operator = self.query_builder.operator;
        let aggregations = aggregation.into_iter().collect();
        self.query_builder.operator = Operator::Window(Window {
            child: Some(Box::new(child_operator)),
            descriptor: self.descriptor,
            aggregations,
            key_fields: self.key_fields,
        });
        self.query_builder
    }
}

#[derive(Debug, Clone)]
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

    pub fn set_sink(&mut self, sink: Sink) {
        self.sink = sink;
    }

    pub fn from_source(source_name: impl Into<String>) -> QueryBuilder {
        QueryBuilder::from_source(source_name)
    }
}

impl QueryBuilder {
    pub fn from_source(source_name: impl Into<String>) -> Self {
        let operator = Operator::LogicalSource {
            source_name: source_name.into(),
        };
        QueryBuilder { operator }
    }

    pub fn sink(self, sink: Sink) -> Query {
        Query {
            sink,
            operator: self.operator,
        }
    }

    pub fn filter(mut self, expression: LogicalExpr) -> Self {
        let child_operator = self.operator;
        self.operator = Operator::Filter(Filter {
            child: Some(Box::new(child_operator)),
            expression,
        });
        self
    }

    // FIXME: This should also support LogicalExpr
    /// Add a `Map` `Operator` to the operator tree. The `Map` `Operator` maps the result of a
    /// computed expression to a new or existing field in the Stream.
    pub fn map(mut self, assigned_field: impl Into<String>, expression: ArithmeticExpr) -> Self {
        let child_operator = self.operator;
        self.operator = Operator::Map(Map {
            child: Some(Box::new(child_operator)),
            assigned_field: assigned_field.into(),
            expression,
        });
        self
    }

    pub fn window(self, descriptor: WindowDescriptor) -> WindowedQueryBuilder {
        WindowedQueryBuilder {
            query_builder: self,
            descriptor,
            key_fields: None,
        }
    }

    pub fn project(mut self, fields: impl IntoIterator<Item = Field>) -> Self {
        let child_operator = self.operator;
        self.operator = Operator::Projection(Projection {
            fields: fields.into_iter().collect(),
            child: Some(Box::new(child_operator)),
        });
        self
    }

    pub fn join_with(self, query: Self) -> JoinWhereBuilder {
        unimplemented!();
    }

    pub fn union(mut self, query: Self) -> Self {
        let child_operator = self.operator;
        self.operator = Operator::Union(Union {
            child: Some(Box::new(child_operator)),
            operators: Box::new(query.operator),
        });
        self
    }

    pub fn rename(self, source_name: impl Into<String>) -> Self {
        unimplemented!();
    }
}
