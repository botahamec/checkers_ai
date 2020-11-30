
mod container;
mod empty;

pub use container::ContainerSpace;
pub use empty::EmptySpace;

/// A Space on a board, that may contain an element
///
/// # Arguments
///
/// - `T` - The element the Space could contain
pub trait Space<T: Sized>{
    fn new() -> Self where Self: Sized; 
}

