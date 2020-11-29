mod space;

use space::Space;

/// Contains a board to play on
///
/// # Arguments
///
/// * `T` - What is contained on the spaces of the board
/// * `SIZE` - The number of rows and columns in the board
struct Board<T, const SIZE: usize> {
	matrix: [[Space<T>; SIZE]; SIZE],
}
