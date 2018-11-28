use cell::Cell;
use cell::CellState;
use coord::ForbiddenPoints;
use coord::Point;
use utils::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

type CellIndex = usize;

#[derive(PartialEq, Clone, Debug)]
pub struct CellNeighbors {
    distance_1: CellIndex,
    distance_2: CellIndex,
}

#[derive(Clone, Debug)]
pub struct EdibleNeighbors {
    pub left: Option<CellNeighbors>,
    pub right: Option<CellNeighbors>,
    pub up: Option<CellNeighbors>,
    pub down: Option<CellNeighbors>,
}

impl EdibleNeighbors {
    pub fn get_valid_edible_neighbors(&self) -> Vec<CellNeighbors> {
        let mut neighbors_direction: Vec<CellNeighbors> = Vec::new();

        if self.left != None { neighbors_direction.push(self.left.clone().unwrap()) }
        if self.right != None { neighbors_direction.push(self.right.clone().unwrap()) }
        if self.up != None { neighbors_direction.push(self.up.clone().unwrap()) }
        if self.down != None { neighbors_direction.push(self.down.clone().unwrap()) }

        neighbors_direction
    }
}

#[derive(Clone)]
pub struct Board {
    origin_cell: Point,
    pub remaining_cells: i16,
    pub width: i16,
    pub height: i16,
    pub cells: Vec<Cell>,
    pub forbidden_points: ForbiddenPoints,
    pub eater_cells: Vec<(usize, EdibleNeighbors)>,
}

impl Board {
    pub fn new(width: i16, height: i16, origin_cell: Point, forbidden_points: ForbiddenPoints) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        assert!(origin_cell.x < width && origin_cell.x >= 0);
        assert!(origin_cell.y < height && origin_cell.y >= 0);

        let cells: Vec<Cell> = vec![Cell::new(); (width * height) as usize];

        Board {
            remaining_cells: (&cells.len() - &forbidden_points.len() - 1) as i16,
            eater_cells: Vec::new(),
            forbidden_points,
            origin_cell,
            width,
            height,
            cells,
        }
    }

    pub fn init_board(&mut self) {
        for (i, cell) in self.cells.iter_mut().enumerate() {
            let (x, y) = coordinates_by_index(i as i16, self.width, self.height);

            if (self.origin_cell.x == x) && (self.origin_cell.y == y) {
                cell.set_dead()
            }

            if self.forbidden_points.contains(&Point{x,y}) {
                cell.set_forbidden()
            }
        }
    }

    fn get_edible_neighbor(&self, x1: i16, y1: i16, x2: i16, y2: i16) -> Option<CellNeighbors> {
        let mut cell_distance_1_is_alive = false;
        let mut cell_distance_2_is_dead = false;
        let is_distance_1_within_boundaries = coordinates_within_boundaries(x1, y1, self.width, self.height);
        let is_distance_2_within_boundaries = coordinates_within_boundaries(x2, y2, self.width, self.height);

        if !(is_distance_1_within_boundaries == is_distance_2_within_boundaries) {
            return None;
        }

        let distance_1_index = index_by_coordinates(x1, y1, self.width);
        let distance_2_index = index_by_coordinates(x2, y2, self.width);

        if let Some(cell) = self.cells.get(distance_1_index) {
            cell_distance_1_is_alive = cell.state == CellState::Alive;
        }

        if let Some(cell) = self.cells.get(distance_2_index) {
            cell_distance_2_is_dead = cell.state == CellState::Dead;
        }

        if cell_distance_1_is_alive && cell_distance_2_is_dead {
            return Some(CellNeighbors {
                distance_1: distance_1_index,
                distance_2: distance_2_index,
            });
        }

        None
    }

    pub fn get_edible_neighbors(&self, index: usize) -> Option<EdibleNeighbors> {
        match self.cells.get(index) {
            Some(cell) => {
                if cell.state != CellState::Alive {
                    return None;
                }

                let (x, y) = coordinates_by_index(index as i16, self.width, self.height);
                let left_edible_neighbors = self.get_edible_neighbor(x - 1, y, x - 2, y);
                let right_edible_neighbors = self.get_edible_neighbor(x + 1, y, x + 2, y);
                let up_edible_neighbors = self.get_edible_neighbor(x, y - 1, x, y - 2);
                let down_edible_neighbors = self.get_edible_neighbor(x, y + 1, x, y + 2);

                if (left_edible_neighbors == None) && (right_edible_neighbors == None) &&
                    (up_edible_neighbors == None) && (down_edible_neighbors == None) {
                    None
                } else {
                    Some(EdibleNeighbors {
                        left: left_edible_neighbors,
                        right: right_edible_neighbors,
                        up: up_edible_neighbors,
                        down: down_edible_neighbors,
                    })
                }
            }
            _ => { None }
        }
    }

    fn eat_cell(&mut self, eater_cell_index: usize, distance1_neighbor: usize, distance2_neighbor: usize) {
        {
            let eater_cell: &mut Cell = self.cells.get_mut(eater_cell_index).unwrap();
            eater_cell.set_dead();
        }
        {
            let distance_1_cell: &mut Cell = self.cells.get_mut(distance1_neighbor).unwrap();
            distance_1_cell.set_dead();
        }
        {
            let distance_2_cell: &mut Cell = self.cells.get_mut(distance2_neighbor).unwrap();
            distance_2_cell.set_alive();
        }

        self.remaining_cells -= 1;
    }

    fn set_eater_cells(&mut self) {
        for (i, _) in self.cells.iter().enumerate() {
            if let Some(edible_neighbor) = self.get_edible_neighbors(i) {
                self.eater_cells.push((i, edible_neighbor));
            }
        }
    }

    // Logic
    pub fn random_eat(&mut self) -> bool {
        self.set_eater_cells();
        let mut has_eaten = false;

        match self.eater_cells.choose_mut(&mut thread_rng()).cloned() {
            Some((cell_index, edible_neighbors)) => {

                if let Some(neighbors) = edible_neighbors.get_valid_edible_neighbors().choose(&mut thread_rng()) {
                    self.eat_cell(cell_index, neighbors.distance_1, neighbors.distance_2);

                    has_eaten = true
                }
            }
            _ => {}
        }
        self.eater_cells.clear();

        has_eaten
    }
}
