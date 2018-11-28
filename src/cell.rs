
#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
    Forbidden
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Cell {state: CellState::Alive }
    }

    pub fn set_alive(&mut self) {
        assert_ne!(self.state, CellState::Forbidden);
        self.state = CellState::Alive;
    }

    pub fn set_dead(&mut self) {
        assert_ne!(self.state, CellState::Forbidden);
        self.state = CellState::Dead;
    }

    pub fn set_forbidden(&mut self) {
        self.state = CellState::Forbidden;
    }

}

