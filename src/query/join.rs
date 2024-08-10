use serde::{Deserialize, Serialize};

use super::{
    expression::Field, operator::Operator, window::window_descriptor::WindowDescriptor,
    QueryBuilder,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Join {
    pub lhs: Field,
    pub rhs: Field,
    pub window: WindowDescriptor,
    pub joined_operators: Box<Operator>,
    pub child: Option<Box<Operator>>,
}

pub struct JoinWhereBuilder {
    query_builder: QueryBuilder,
}

pub struct JoinEqualsBuilder {
    query_builder: QueryBuilder,
}

pub struct JoinWindowBuilder {
    query_builder: QueryBuilder,
}

impl JoinWhereBuilder {
    pub fn where_field(self, field: impl Into<Field>) -> JoinEqualsBuilder {
        unimplemented!()
    }
}

impl JoinEqualsBuilder {
    pub fn equals(self, field: impl Into<Field>) -> JoinWindowBuilder {
        unimplemented!()
    }
}

impl JoinWindowBuilder {
    pub fn window(self, descriptor: WindowDescriptor) -> QueryBuilder {
        unimplemented!()
    }
}
