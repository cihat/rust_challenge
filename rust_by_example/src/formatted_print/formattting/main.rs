use std::fmt::{self, Display, Formatter};

struct City {
  name: &'static str,
  lat: f32,
  lon: f32,
}

impl Display for City {}
