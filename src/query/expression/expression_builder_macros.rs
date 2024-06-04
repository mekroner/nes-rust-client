macro_rules! cmp_operator {
    ($name:ident, $operator:expr) => {
        pub fn $name(mut self, other: Self) -> Self {
            let data_type =
                match NESType::try_resolve(self.expr.data_type(), other.expr.data_type()) {
                    Some(_) => NESType::Bool,
                    None => {
                        self.error = Some(ExprBuildError {});
                        NESType::Undefined
                    }
                };
            self.expr = RawExpr::Binary(BinaryExpr {
                lhs: Box::new(self.expr),
                rhs: Box::new(other.expr),
                operator: $operator,
                data_type,
            });
            self
        }
    };
}

macro_rules! boolean_operator {
    ($name:ident, $operator:expr) => {
        pub fn $name(mut self, other: Self) -> Self {
            let data_type =
                match NESType::try_resolve(self.expr.data_type(), other.expr.data_type()) {
                    Some(NESType::Bool) | Some(NESType::Undefined) => NESType::Bool,
                    _ => {
                        self.error = Some(ExprBuildError {});
                        NESType::Undefined
                    }
                };
            self.expr = RawExpr::Binary(BinaryExpr {
                lhs: Box::new(self.expr),
                rhs: Box::new(other.expr),
                operator: $operator,
                data_type,
            });
            self
        }
    };
}

pub(super) use boolean_operator;
pub(super) use cmp_operator;
