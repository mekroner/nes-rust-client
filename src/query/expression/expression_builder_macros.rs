macro_rules! cmp_operator {
    ($name:ident, $operator:expr) => {
        pub fn $name(mut self, other: Self) -> Self {
            let data_type =
                match nes_types::NesType::try_resolve(self.expr.data_type(), other.expr.data_type()) {
                    Some(_) => nes_types::NesType::Bool,
                    None => {
                        self.error = Some(ExprBuildError {});
                        nes_types::NesType::Undefined
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
                match nes_types::NesType::try_resolve(self.expr.data_type(), other.expr.data_type()) {
                    Some(nes_types::NesType::Bool) | Some(nes_types::NesType::Undefined) => nes_types::NesType::Bool,
                    _ => {
                        self.error = Some(ExprBuildError {});
                        nes_types::NesType::Undefined
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
