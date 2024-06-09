use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct OxidizedInstant {
    current_time_ms: f64,
}

impl OxidizedInstant {
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        OxidizedInstant {
            current_time_ms: now.as_secs_f64() * 1000.0,
        }
    }

    pub fn duration_since(&self, earlier: OxidizedInstant) -> Duration {
        let diff_ms = self.current_time_ms - earlier.current_time_ms;
        Duration::from_secs_f64(diff_ms / 1000.0)
    }

    pub fn elapsed(&self) -> Duration {
        let now = OxidizedInstant::now();
        now.duration_since(*self)
    }

    pub fn add_duration(&self, duration: Duration) -> Self {
        OxidizedInstant {
            current_time_ms: self.current_time_ms + duration.as_secs_f64() * 1000.0,
        }
    }

    pub fn sub_duration(&self, duration: Duration) -> Self {
        OxidizedInstant {
            current_time_ms: self.current_time_ms - duration.as_secs_f64() * 1000.0,
        }
    }
}

impl Add<Duration> for OxidizedInstant {
    type Output = OxidizedInstant;

    fn add(self, other: Duration) -> OxidizedInstant {
        self.add_duration(other)
    }
}

impl Sub<Duration> for OxidizedInstant {
    type Output = OxidizedInstant;

    fn sub(self, other: Duration) -> OxidizedInstant {
        self.sub_duration(other)
    }
}

impl Sub<OxidizedInstant> for OxidizedInstant {
    type Output = Duration;

    fn sub(self, other: OxidizedInstant) -> Duration {
        self.duration_since(other)
    }
}

impl PartialEq for OxidizedInstant {
    fn eq(&self, other: &Self) -> bool {
        self.current_time_ms == other.current_time_ms
    }
}

impl Eq for OxidizedInstant {}

impl PartialOrd for OxidizedInstant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.current_time_ms.partial_cmp(&other.current_time_ms)
    }
}

impl Ord for OxidizedInstant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.current_time_ms.partial_cmp(&other.current_time_ms).unwrap()
    }
}
