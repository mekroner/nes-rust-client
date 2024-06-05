use crate::query::expression::Field;

#[derive(Debug, Clone, Copy)]
pub enum AggregationType {
    Sum,
    Average,
    Min,
    Max,
    Median,
    Count,
}

#[derive(Debug)]
pub struct Aggregation {
    field: Option<Field>,
    projected_field: Option<Field>,
    agg_type: AggregationType,
}

impl Aggregation {
    pub fn count(field: impl Into<Field>) -> Self {
        Self {
            field: Some(field.into()),
            projected_field: None,
            agg_type: AggregationType::Count,
        }
    }

    pub fn sum() -> Self {
        Self {
            field: None,
            projected_field: None,
            agg_type: AggregationType::Sum,
        }
    }

    pub fn average() -> Self {
        Self {
            field: None,
            projected_field: None,
            agg_type: AggregationType::Average,
        }
    }

    pub fn min() -> Self {
        Self {
            field: None,
            projected_field: None,
            agg_type: AggregationType::Min,
        }
    }

    pub fn max() -> Self {
        Self {
            field: None,
            projected_field: None,
            agg_type: AggregationType::Max,
        }
    }

    pub fn median() -> Self {
        Self {
            field: None,
            projected_field: None,
            agg_type: AggregationType::Median,
        }
    }

    pub fn on_field(mut self, field: impl Into<Field>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn as_field(mut self, field: impl Into<Field>) -> Self {
        self.projected_field = Some(field.into());
        self
    }

    pub fn field(&self) -> Option<&Field> {
        self.field.as_ref()
    }

    pub fn agg_type(&self) -> AggregationType {
        self.agg_type
    }

    pub fn projected_field(&self) -> Option<&Field> {
        self.projected_field.as_ref()
    }
}
