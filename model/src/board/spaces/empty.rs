
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Space;

/// A space in a board, doesn't contain an element
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmptySpace {}

impl EmptySpace {
    fn new() -> Self {
        EmptySpace {}
    } 
}

impl<T> Space<T> for EmptySpace {
    /// Creates an empty space
    fn new() -> Self {
        EmptySpace::new()
    }
}

impl fmt::Display for EmptySpace {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("| |")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::any::Any;

    fn is<T: 'static>(obj: &dyn Any) -> bool {
		obj.is::<T>()
	}

    #[test]
    fn new() {
        let cut = EmptySpace::new();
        assert!(is::<EmptySpace>(&cut));
    }

    #[test]
    fn to_string() {
        let cut = EmptySpace {};
        assert_eq!(cut.to_string(), "| |");
    }
}
