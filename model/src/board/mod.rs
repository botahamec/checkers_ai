mod spaces;

use spaces::Space;

/// Contains a board to play on
///
/// # Arguments
///
/// * `T` - What is contained on the spaces of the board
/// * `SIZE` - The number of rows and columns in the board
struct Board<'a, T: Copy, const SIZE: usize> {
	matrix: [[&'a dyn Space<T>; SIZE]; SIZE],
}
