use std::{fmt::Display, net::SocketAddr};

use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Addr(String);

impl Addr {
    pub fn new(address: SocketAddr) -> Self {
        let string = format!("{address:?}");
        Self(string)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(u64);

impl Id {
    pub fn from_now() -> Self {
        let ms = chrono::Utc::now().timestamp_millis() as u64;
        let total = (ms * 1000) + rand_range(1000);
        Self(total)
    }

    pub fn from_string(input: &str) -> Option<Self> {
        let inner: u64 = input.parse().ok()?;
        Some(Self(inner))
    }

    pub fn from_u64(input: u64) -> Self {
        Self(input)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Id {
    fn from(input: u64) -> Self {
        Self::from_u64(input)
    }
}

#[test]
fn test_string_convertion() {
    let id = Id::from_now();
    let str = id.to_string();
    assert_eq!(id, Id::from_string(&str).unwrap());
}

fn rand_range(n: u64) -> u64 {
    let random: u64 = random();
    random % n
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.0;
        let padded = format!("{inner:0>20}"); // pads to the left to make 20 chars of length
        f.write_str(&padded)
    }
}

#[test]
fn length_of_max() {
    assert_eq!(u64::MAX, 18446744073709551615_u64);
    assert_eq!(20, "18446744073709551615".len())
}
