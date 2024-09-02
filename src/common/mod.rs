mod tests;

/// Position struct - represents a position on the grid (x: i32, y: i32)
/// ### Example:
/// Position(0, 0) - represents the top left corner of the grid
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}

/// Grid trait - represents a grid
/// - A grid is a 2D structure that can be represented by a width and a height
/// - A grid can be a map, a game board, etc.
/// - A grid has a width and a height
/// - A grid can check if a position is inside the grid
pub trait Grid<T> {
    /// Returns elements of the grid
    fn elements(&self) -> &Vec<T>;
    /// Returns the width of the grid
    fn width(&self) -> i32;
    /// Returns the height of the grid
    fn height(&self) -> i32;
    /// Returns true if the position is inside the grid, false otherwise
    fn is_inside(&self, position: &Position) -> bool {
        position.x() >= 0
            && position.x() < self.width()
            && position.y() >= 0
            && position.y() < self.height()
    }
    /// Returns positions of the neighbors of the given position
    fn get_neighbors_positions(&self, position: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let deltas = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        for delta in deltas {
            let x = position.x() + delta.0;
            let y = position.y() + delta.1;
            let neighbor = Position(x, y);
            if self.is_inside(&neighbor) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
    /// Returns indices of the neighbors of the given position
    fn get_neighbors_indices(&self, position: &Position) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let deltas = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        for delta in deltas {
            let x = position.x() + delta.0;
            let y = position.y() + delta.1;
            let neighbor = Position(x, y);
            if self.is_inside(&neighbor) {
                let index = (y * self.width() + x) as usize;
                neighbors.push(index);
            }
        }
        neighbors
    }
}

/// SquareGrid struct - represents a square grid
pub struct SquareGrid<T> {
    width: i32,
    height: i32,
    elements: Vec<T>,
}

impl<T> SquareGrid<T> {
    pub fn new(width: i32, height: i32, elements: Vec<T>) -> Self {
        SquareGrid {
            width,
            height,
            elements,
        }
    }
}

impl<T> Grid<T> for SquareGrid<T> {
    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}
