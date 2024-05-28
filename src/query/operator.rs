use std::{default, fmt::Debug, thread::current};

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
        OperatorIterator { current: Some(self) }
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

#[derive(Debug)]
pub enum LogicalExpression {
    Attribute(String),
    Literal(i32),
    Equal(Box<LogicalExpression>, Box<LogicalExpression>),
    NotEqual(Box<LogicalExpression>, Box<LogicalExpression>),
    And(Box<LogicalExpression>, Box<LogicalExpression>),
    Or(Box<LogicalExpression>, Box<LogicalExpression>),
}
//
// #[derive(Debug)]
// pub enum ArithmeticExpression {
//     Attribute(String),
//     Literal(i32),
// }

#[derive(Debug)]
pub enum TimeCharacteristic {
    EventTime { field_name: String, unit: TimeUnit },
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TimeUnit {
    #[default]
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnit {
    pub fn to_string(&self) -> String {
        match self {
            TimeUnit::Milliseconds => "Milliseconds",
            TimeUnit::Seconds => "Seconds",
            TimeUnit::Minutes => "Minutes",
            TimeUnit::Hours => "Hours",
            TimeUnit::Days => "Days",
        }
        .to_string()
    }

    pub fn to_scalar(&self) -> u32 {
        match self {
            TimeUnit::Milliseconds => 1,
            TimeUnit::Seconds => 1000,
            TimeUnit::Minutes => 1000 * 60,
            TimeUnit::Hours => 1000 * 60 * 60,
            TimeUnit::Days => 1000 * 60 * 60 * 24,
        }
    }
}

#[derive(Debug)]
pub struct Duration {
    amount: u32,
    unit: TimeUnit,
}

impl Duration {
    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn unit(&self) -> TimeUnit {
        self.unit
    }

    pub fn milliseconds(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Milliseconds,
        }
    }

    pub fn seconds(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Seconds,
        }
    }

    pub fn minutes(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Minutes,
        }
    }

    pub fn hours(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Hours,
        }
    }

    pub fn days(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Days,
        }
    }
}

#[derive(Debug)]
pub enum WindowDescriptor {
    TumblingWindow {
        duration: Duration,
        time_character: TimeCharacteristic,
    },
    // SlidingWindow {
    //     time_character: TimeCharacteristic,
    //     size: Duration,
    //     slide: Duration,
    // },
    // ThresholdWindow {
    //     duration: Duration,
    //     time_character: TimeCaracteristic,
    // },
}
