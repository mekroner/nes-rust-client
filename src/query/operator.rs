use std::fmt::Display;
use crate::expression::field::Field;

use serde::{Deserialize, Serialize};

use super::{
    join::Join,
    window::{aggregation::Aggregation, window_descriptor::WindowDescriptor},
};
use crate::expression::{ArithmeticExpr, LogicalExpr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub expression: LogicalExpr,
    pub child: Option<Box<Operator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub assigned_field: String,
    pub expression: ArithmeticExpr,
    pub child: Option<Box<Operator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub descriptor: WindowDescriptor,
    pub aggregations: Vec<Aggregation>,
    pub key_fields: Option<Vec<String>>,
    pub child: Option<Box<Operator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Union {
    pub operators: Box<Operator>,
    pub child: Option<Box<Operator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projection {
    pub fields: Vec<Field>,
    pub child: Option<Box<Operator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    LogicalSource { source_name: String },
    Projection(Projection),
    Filter(Filter),
    Map(Map),
    Window(Window),
    Join(Join),
    Union(Union),
}

impl Operator {
    pub fn child(&self) -> Option<&Operator> {
        match self {
            Operator::LogicalSource { .. } => None,
            Operator::Projection(Projection{child, ..}) => child.as_deref(),
            Operator::Filter(Filter { child, .. }) => child.as_deref(),
            Operator::Map(Map { child, .. }) => child.as_deref(),
            Operator::Window(Window { child, .. }) => child.as_deref(),
            Operator::Join(Join { child, .. }) => child.as_deref(),
            Operator::Union(Union { child, .. }) => child.as_deref(),
        }
    }

    pub fn has_child(&self) -> bool {
        self.child().is_some()
    }

    pub fn iter(&self) -> OperatorIterator {
        OperatorIterator {
            current: Some(self),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::LogicalSource { source_name } => write!(f, "LogicalSource({source_name})"),
            Operator::Projection(_) => write!(f, "Project(TODO!!!)"),
            Operator::Filter(_) => write!(f, "Filter(TODO!!!)"),
            Operator::Map(_) => write!(f, "Map(TODO!!!)"),
            Operator::Window(_) => write!(f, "Window(TODO!!!)"),
            Operator::Join(_) => write!(f, "Join(TODO!!!)"),
            Operator::Union(_) => write!(f, "Union(TODO!!!)"),
        }
    }
}

pub struct OperatorIterator<'a> {
    current: Option<&'a Operator>,
}

impl<'a> Iterator for OperatorIterator<'a> {
    type Item = &'a Operator;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current?.child();
        current
    }
}

#[cfg(test)]
mod operator_tests {
    use super::Operator as O;
    // #[test]
    // fn test_operator_iter() {
    //     let operators = O::Filter(Filter {
    //         expression: LE::Equal(
    //             Box::new(LE::Attribute("value".to_string())),
    //             Box::new(LE::Literal(0)),
    //         ),
    //         child: Some(Box::new(O::LogicalSource {
    //             source_name: "default".to_string(),
    //         })),
    //     });
    //     let mut iter = operators.iter();
    //     assert!(matches!(iter.next(), Some(O::Filter { .. })));
    //     assert!(matches!(iter.next(), Some(O::LogicalSource { .. })));
    //     assert!(matches!(iter.next(), None));
    //     assert!(matches!(iter.next(), None));
    // }
}
