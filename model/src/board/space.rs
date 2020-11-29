use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A space in a board, that may or may not contain an element
///
/// # Arguments
///
/// * `T` - What the space may contain
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Space<T> {
	element: Option<T>,
}

impl<T> Space<T> {
	/// Creates an empty space
	pub fn new() -> Self {
		Space { element: None }
	}

	/// Creates a new space containing the specified element
	pub fn with_element(element: T) -> Self {
		Space {
			element: Some(element),
		}
	}

	/// Gets the element from the space. Returns None if the Space is empty
	pub fn element(&self) -> &Option<T> {
		&self.element
	}

	/// Checks if the space is empty or not
	pub fn is_empty(&self) -> bool {
		self.element().is_none()
	}

	/// True if the space contains some element
	pub fn has_element(&self) -> bool {
		self.element().is_some()
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

impl<T: fmt::Display> fmt::Display for Space<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!(
			"|{}|",
			match self.element() {
				Some(piece) => piece.to_string(),
				None => String::from(" "),
			}
		))
	}
}

impl<T> From<Option<T>> for Space<T> {
	fn from(option: Option<T>) -> Self {
		Space { element: option }
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
		let cut = Space::<u8>::new();
		assert_eq!(cut.element, None);
	}

	#[test]
	fn with_element() {
		const ELEMENT: char = 'a';
		let cut = Space::with_element(ELEMENT);
		assert!(cut.element.is_some());
		assert!(is::<char>(&cut.element.unwrap()));
		assert_eq!(cut.element.unwrap(), ELEMENT);
	}

	#[test]
	fn element() {
		// test with None
		let element = None;
		let cut = Space::<u8> { element };
		assert_eq!(cut.element(), &element);

		// test with a value
		let element = Some(5);
		let cut = Space { element };
		assert_eq!(cut.element(), &element);
	}

	#[test]
	fn is_empty() {
		// test true
		let cut = Space::<u8> { element: None };
		assert!(cut.is_empty());

		// test false
		let element = Some(5);
		let cut = Space { element };
		assert!(!cut.is_empty());
	}

	#[test]
	fn has_element() {
		// test truth
		let element = Some(5);
		let cut = Space { element };
		assert!(cut.has_element());

		// test falsity
		let cut = Space::<u8> { element: None };
		assert!(!cut.has_element());
	}

	#[test]
	fn set_optional_element() {
		// test with None
		let mut cut = Space::new();
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
		let mut cut = Space::new();
		cut.set_element(ELEMENT);
		assert!(cut.element.is_some());
		assert_eq!(cut.element.unwrap(), ELEMENT);
	}

	#[test]
	fn clear() {
		let mut cut = Space::<u8> { element: Some(67) };
		cut.clear();
		assert_eq!(cut.element, None);
	}

	#[test]
	fn to_string() {
		// test None
		let cut = Space::<u8> { element: None };
		assert_eq!(cut.to_string(), String::from("| |"));

		// test 5
		let cut = Space::<u8> { element: Some(5) };
		assert_eq!(cut.to_string(), String::from("|5|"));
	}

	#[test]
	fn from_option() {
		// test None
		let element: Option<u8> = None;
		let cut = Space::from(element);
		assert_eq!(cut.element, element);

		// test 5
		let element = Some(5);
		let cut = Space::from(element);
		assert_eq!(cut.element, element);
	}
}
