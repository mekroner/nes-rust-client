use super::{
    expression::LogicalExpression,
    window::{
        aggregation::Aggregation, window_descriptor::WindowDescriptor,
    },
};

#[derive(Debug)]
pub enum Operator {
    LogicalSource {
        source_name: String,
    },
    Filter {
        expression: LogicalExpression,
        child: Option<Box<Operator>>,
    },
    Window {
        descriptor: WindowDescriptor,
        aggregations: Vec<Aggregation>,
        key_fields: Option<Vec<String>>,
        child: Option<Box<Operator>>,
    },
}

impl Operator {
    pub fn child(&self) -> Option<&Operator> {
        match self {
            Operator::LogicalSource { .. } => None,
            Operator::Filter { child, .. } => child.as_deref(),
            Operator::Window { child, .. } => child.as_deref(),
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
    use super::{LogicalExpression as LE, Operator as O};
    #[test]
    fn test_operator_iter() {
        let operators = O::Filter {
            child: Some(Box::new(O::LogicalSource {
                source_name: "default".to_string(),
            })),
            expression: LE::Equal(
                Box::new(LE::Attribute("value".to_string())),
                Box::new(LE::Literal(0)),
            ),
        };
        let mut iter = operators.iter();
        assert!(matches!(iter.next(), Some(O::Filter { .. })));
        assert!(matches!(iter.next(), Some(O::LogicalSource { .. })));
        assert!(matches!(iter.next(), None));
        assert!(matches!(iter.next(), None));
    }
}
