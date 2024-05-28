use super::time::{Duration, TimeCharacteristic};

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
