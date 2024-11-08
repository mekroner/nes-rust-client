use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeCharacteristic {
    EventTime { field_name: String, unit: TimeUnit },
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
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

    pub const fn to_scalar(&self) -> u32 {
        match self {
            TimeUnit::Milliseconds => 1,
            TimeUnit::Seconds => 1000,
            TimeUnit::Minutes => 1000 * 60,
            TimeUnit::Hours => 1000 * 60 * 60,
            TimeUnit::Days => 1000 * 60 * 60 * 24,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    pub amount: u32,
    pub unit: TimeUnit,
}

impl Duration {
    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn unit(&self) -> TimeUnit {
        self.unit
    }

    pub fn to_milliseconds(&self) -> u32 {
        self.amount * self.unit.to_scalar()
    }

    pub fn from_milliseconds(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Milliseconds,
        }
    }

    pub fn from_seconds(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Seconds,
        }
    }

    pub fn from_minutes(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Minutes,
        }
    }

    pub fn from_hours(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Hours,
        }
    }

    pub fn from_days(amount: u32) -> Self {
        Self {
            amount,
            unit: TimeUnit::Days,
        }
    }
}
