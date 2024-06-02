
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
    pub field_name: String,
    pub projected_field_name: Option<String>,
    pub agg_type: AggregationType,
}
