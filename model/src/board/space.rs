use std::fmt;

/// A space in a board, that may or may not contain an element
///
/// # Arguments
///
/// * `T` - What the space may contain
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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
	pub fn element(self) -> Option<T> {
		self.element
	}

	/// Checks if the space is empty or not
	pub fn is_empty(self) -> bool {
		self.element().is_none()
	}

	/// True if the space contains some element
	pub fn has_element(self) -> bool {
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
			match &self.element {
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
