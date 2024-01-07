use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::ops::Mul;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Multiplier(pub f32);

impl Mul for Multiplier {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self(self.0 * rhs.0)
	}
}

impl PartialEq<f32> for Multiplier {
	fn eq(&self, other: &f32) -> bool {
		self.0 == *other
	}
}

impl PartialOrd<f32> for Multiplier {
	fn partial_cmp(&self, rhs: &f32) -> Option<Ordering> {
		self.0.partial_cmp(rhs)
	}
}

impl Display for Multiplier {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "x{}", self.0)
	}
}