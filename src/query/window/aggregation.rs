use serde::{Deserialize, Serialize};

use crate::expression::Field;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AggregationType {
    Sum,
    Average,
    Min,
    Max,
    Median,
    Count,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    field: Option<Field>,
    projected_field: Option<Field>,
    agg_type: AggregationType,
}

impl Aggregation {
    pub fn count() -> Self {
        Self {
            field: None,
            projected_field: None,
            agg_type: AggregationType::Count,
        }
    }

    pub fn sum(field: impl Into<Field>) -> Self {
        Self {
            field: Some(field.into()),
            projected_field: None,
            agg_type: AggregationType::Sum,
        }
    }

    pub fn average(field: impl Into<Field>) -> Self {
        Self {
            field: Some(field.into()),
            projected_field: None,
            agg_type: AggregationType::Average,
        }
    }

    pub fn min(field: impl Into<Field>) -> Self {
        Self {
            field: Some(field.into()),
            projected_field: None,
            agg_type: AggregationType::Min,
        }
    }

    pub fn max(field: impl Into<Field>) -> Self {
        Self {
            field: Some(field.into()),
            projected_field: None,
            agg_type: AggregationType::Max,
        }
    }

    pub fn median(field: impl Into<Field>) -> Self {
        Self {
            field: Some(field.into()),
            projected_field: None,
            agg_type: AggregationType::Median,
        }
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
