use serde::{Deserialize, Serialize};

use crate::query::time::{Duration, TimeCharacteristic};

#[derive(Debug, Clone, Serialize, Deserialize) ]
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
