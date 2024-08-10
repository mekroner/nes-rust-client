use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueryState {
    Registered,
    Optimizing,
    MarkedForDeployment,
    MarkedForRedeployment,
    MarkedForMigration,
    Deployed,
    Redeployed,
    Running,
    Migrating,
    MarkedForHardStop,
    MarkedForSoftStop,
    SoftStopTriggered,
    SoftStopCompleted,
    Stopped,
    MarkedForFailure,
    Failed,
    Restarting,
    MigrationCompleted,
    Explained,
}

impl TryFrom<&String> for QueryState {
    type Error = QueryStateParseError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        QueryState::try_from(value.as_str())
    }
}

impl TryFrom<String> for QueryState {
    type Error = QueryStateParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        QueryState::try_from(value.as_str())
    }
}

impl TryFrom<&str> for QueryState {
    type Error = QueryStateParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "REGISTERED" => Ok(Self::Registered),
            "OPTIMIZING" => Ok(Self::Optimizing),
            "MARKED_FOR_DEPLOYMENT" => Ok(Self::MarkedForDeployment),
            "MARKED_FOR_REDEPLOYMENT" => Ok(Self::MarkedForRedeployment),
            "MARKED_FOR_MIGRATION" => Ok(Self::MarkedForMigration),
            "DEPLOYED" => Ok(Self::Deployed),
            "REDEPLOYED" => Ok(Self::Redeployed),
            "RUNNING" => Ok(Self::Running),
            "MIGRATING" => Ok(Self::Migrating),
            "MARKED_FOR_HARD_STOP" => Ok(Self::MarkedForHardStop),
            "MARKED_FOR_SOFT_STOP" => Ok(Self::MarkedForSoftStop),
            "SOFT_STOP_TRIGGERED" => Ok(Self::SoftStopTriggered),
            "SOFT_STOP_COMPLETED" => Ok(Self::SoftStopCompleted),
            "STOPPED" => Ok(Self::Stopped),
            "MARKED_FOR_FAILURE" => Ok(Self::MarkedForFailure),
            "FAILED" => Ok(Self::Failed),
            "RESTARTING" => Ok(Self::Restarting),
            "MIGRATION_COMPLETED" => Ok(Self::MigrationCompleted),
            "EXPLAINED" => Ok(Self::Explained),
            _ => Err(QueryStateParseError(value.to_string())),
        }
    }
}

impl Display for QueryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            QueryState::Registered => "Registered",
            QueryState::Optimizing => "Optimizing",
            QueryState::MarkedForDeployment => "MarkedForDeployment",
            QueryState::MarkedForRedeployment => "MarkedForRedeployment",
            QueryState::MarkedForMigration => "MarkedForMigration",
            QueryState::Deployed => "Deployed",
            QueryState::Redeployed => "Redeployed",
            QueryState::Running => "Running",
            QueryState::Migrating => "Migrating",
            QueryState::MarkedForHardStop => "MarkedForHardStop",
            QueryState::MarkedForSoftStop => "MarkedForSoftStop",
            QueryState::SoftStopTriggered => "SoftStopTriggered",
            QueryState::SoftStopCompleted => "SoftStopCompleted",
            QueryState::Stopped => "Stopped",
            QueryState::MarkedForFailure => "MarkedForFailure",
            QueryState::Failed => "Failed",
            QueryState::Restarting => "Restarting",
            QueryState::MigrationCompleted => "MigrationCompleted",
            QueryState::Explained => "Explained",
        };
        write!(f, "{str}")
    }
}

#[derive(Debug)]
pub struct QueryStateParseError(String);

impl Display for QueryStateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse {} as Query State. Unknown State!",
            self.0
        )
    }
}
