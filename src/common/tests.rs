use super::{Grid, Position, SquareGrid};

#[test]
fn test_square_grid_neighbors_positions() {
    let elements = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let grid = SquareGrid::new(3, 3, elements);

    let position = Position(0, 0);
    let neighbors = grid.get_neighbors_positions(&position);

    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0], Position(1, 0));
    assert_eq!(neighbors[1], Position(0, 1));
    assert_eq!(neighbors[2], Position(1, 1));

    let position = Position(1, 0);
    let neighbors = grid.get_neighbors_positions(&position);

    assert_eq!(neighbors.len(), 5);
    assert_eq!(neighbors[0], Position(0, 0));
    assert_eq!(neighbors[1], Position(2, 0));
    assert_eq!(neighbors[2], Position(1, 1));
    assert_eq!(neighbors[3], Position(0, 1));
    assert_eq!(neighbors[4], Position(2, 1));

    let position = Position(2, 0);
    let neighbors = grid.get_neighbors_positions(&position);

    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0], Position(1, 0));
    assert_eq!(neighbors[1], Position(2, 1));
    assert_eq!(neighbors[2], Position(1, 1));
}

#[test]
fn test_square_grid_neighbors_indices() {
    let elements = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let grid = SquareGrid::new(3, 3, elements);

    let position = Position(0, 0);
    let neighbors = grid.get_neighbors_indices(&position);

    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0], 1);
    assert_eq!(neighbors[1], 3);
    assert_eq!(neighbors[2], 4);

    let position = Position(1, 0);
    let neighbors = grid.get_neighbors_indices(&position);

    assert_eq!(neighbors.len(), 5);
    assert_eq!(neighbors[0], 0);
    assert_eq!(neighbors[1], 2);
    assert_eq!(neighbors[2], 4);
    assert_eq!(neighbors[3], 3);
    assert_eq!(neighbors[4], 5);

    let position = Position(2, 0);
    let neighbors = grid.get_neighbors_indices(&position);

    assert_eq!(neighbors.len(), 3);
    assert_eq!(neighbors[0], 1);
    assert_eq!(neighbors[1], 5);
    assert_eq!(neighbors[2], 4);
}
