use super::{
    expression::LogicalExpr,
    join::Join,
    window::{aggregation::Aggregation, window_descriptor::WindowDescriptor},
};

#[derive(Debug)]
pub struct Filter {
    pub expression: LogicalExpr,
    pub child: Option<Box<Operator>>,
}

#[derive(Debug)]
pub enum Operator {
    LogicalSource {
        source_name: String,
    },
    Filter(Filter),
    Window {
        descriptor: WindowDescriptor,
        aggregations: Vec<Aggregation>,
        key_fields: Option<Vec<String>>,
        child: Option<Box<Operator>>,
    },
    // Join(Join),
}

impl Operator {
    pub fn child(&self) -> Option<&Operator> {
        match self {
            Operator::LogicalSource { .. } => None,
            Operator::Filter(Filter { child, .. }) => child.as_deref(),
            Operator::Window { child, .. } => child.as_deref(),
            // Operator::Join(Join { child, .. }) => child.as_deref(),
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
