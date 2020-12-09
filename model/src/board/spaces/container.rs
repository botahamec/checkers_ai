use std::error::Error;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Space;

/// There is no element at the given space
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct NoElementError;

impl fmt::Display for NoElementError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "The space did not have an element")
	}
}

impl Error for NoElementError {}

/// A space in a board, that may or may not contain an element
///
/// # Arguments
///
/// * `T` - What the space may contain
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContainerSpace<T> {
	element: Option<T>,
}

impl<T> Space<T> for ContainerSpace<T> {
	/// Creates an empty space
	fn new() -> Self {
		ContainerSpace { element: None }
	}
}

impl<T: Copy> ContainerSpace<T> {
    /// Gets the element, or returns an error
	pub const fn element(&self) -> Result<T, NoElementError> {
		match self.as_option() {
			Some(p) => Ok(*p),
			None => Err(NoElementError),
		}
	}
}

impl<T> ContainerSpace<T> {
	/// Creates a new space containing the specified element
	pub const fn with_element(element: T) -> Self {
		ContainerSpace {
			element: Some(element),
		}
	}

	/// Gets the element from the space. Returns None if the Space is empty
	pub const fn as_option(&self) -> &Option<T> {
		&self.element
	}

	/// Checks if the space is empty or not
	pub const fn is_empty(&self) -> bool {
		self.as_option().is_none()
	}

	/// True if the space contains some element
	pub const fn has_element(&self) -> bool {
		self.as_option().is_some()
	}

	/// Sets the element to the given piece, or clears it
	pub fn set_optional_element(&mut self, option: Option<T>) {
		self.element = option;
	}

	/// Sets the element of the space
	pub fn set_element(&mut self, element: T) {
		self.set_optional_element(Some(element));
	}

	/// Clears the element from the space, making it empty
	pub fn clear(&mut self) {
		self.set_optional_element(None);
	}
}

impl<T: fmt::Display> fmt::Display for ContainerSpace<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!(
			"|{}|",
			match self.as_option() {
				Some(piece) => piece.to_string(),
				None => String::from(" "),
			}
		))
	}
}

impl<T> From<Option<T>> for ContainerSpace<T> {
	fn from(option: Option<T>) -> Self {
		ContainerSpace { element: option }
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
	fn no_element_err_to_str() {
		assert_eq!(
			NoElementError.to_string(),
			"The space did not have an element"
		);
	}

	#[test]
	fn new() {
		let cut = ContainerSpace::<u8>::new();
		assert_eq!(cut.element, None);
	}

	#[test]
	fn with_element() {
		const ELEMENT: char = 'a';
		let cut = ContainerSpace::with_element(ELEMENT);
		assert!(cut.element.is_some());
		assert!(is::<char>(&cut.element.unwrap()));
		assert_eq!(cut.element.unwrap(), ELEMENT);
	}

	#[test]
	fn as_option() {
		// test with None
		let element = None;
		let cut = ContainerSpace::<u8> { element };
		assert_eq!(cut.as_option(), &element);

		// test with a value
		let element = Some(5);
		let cut = ContainerSpace { element };
		assert_eq!(cut.as_option(), &element);
	}

	#[test]
	fn element() {
		// test with None
		let element = None;
		let cut = ContainerSpace::<u8> { element };
		assert!(cut.element().is_err());
		assert_eq!(cut.element(), Err(NoElementError));

		// test with a value
		const VALUE: u8 = 5;
		let cut = ContainerSpace { element: Some(VALUE) };
		assert!(cut.element().is_ok());
		assert_eq!(cut.element().unwrap(), VALUE);
	}

	#[test]
	fn is_empty() {
		// test true
		let cut = ContainerSpace::<u8> { element: None };
		assert!(cut.is_empty());

		// test false
		let element = Some(5);
		let cut = ContainerSpace { element };
		assert!(!cut.is_empty());
	}

	#[test]
	fn has_element() {
		// test truth
		let element = Some(5);
		let cut = ContainerSpace { element };
		assert!(cut.has_element());

		// test falsity
		let cut = ContainerSpace::<u8> { element: None };
		assert!(!cut.has_element());
	}

	#[test]
	fn set_optional_element() {
		// test with None
		let mut cut = ContainerSpace::new();
		let element = None;
		cut.set_optional_element(element);
		assert_eq!(cut.element, element);

		// test with 5
		let element = Some(5);
		cut.set_optional_element(element);
		assert_eq!(cut.element, element);
	}

	#[test]
	fn set_element() {
		const ELEMENT: u8 = 5;
		let mut cut = ContainerSpace::new();
		cut.set_element(ELEMENT);
		assert!(cut.element.is_some());
		assert_eq!(cut.element.unwrap(), ELEMENT);
	}

	#[test]
	fn clear() {
		let mut cut = ContainerSpace::<u8> { element: Some(67) };
		cut.clear();
		assert_eq!(cut.element, None);
	}

	#[test]
	fn to_string() {
		// test None
		let cut = ContainerSpace::<u8> { element: None };
		assert_eq!(cut.to_string(), String::from("| |"));

		// test 5
		let cut = ContainerSpace::<u8> { element: Some(5) };
		assert_eq!(cut.to_string(), String::from("|5|"));
	}

	#[test]
	fn from_option() {
		// test None
		let element: Option<u8> = None;
		let cut = ContainerSpace::from(element);
		assert_eq!(cut.element, element);

		// test 5
		let element = Some(5);
		let cut = ContainerSpace::from(element);
		assert_eq!(cut.element, element);
	}
}
