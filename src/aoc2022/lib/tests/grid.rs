use crate::aoc2022::lib::grid::Grid2D;

#[test]
fn grid_test() {
    let grid = Grid2D::create(10, 10);
    assert_eq!(grid.size(), 100, "invalid size");
}