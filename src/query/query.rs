use super::{
    expression::LogicalExpr,
    operator::{Operator, OperatorIterator},
    sink::Sink,
    window::WindowDescriptor,
};

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

    pub fn filter(mut self, expression: LogicalExpr) -> Self {
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
