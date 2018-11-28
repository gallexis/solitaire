use cell;

pub fn display_cell(cell: &cell::Cell) -> String{
    match cell.state {
        cell::CellState::Alive => " 0 ".to_string(),
        cell::CellState::Dead => " _ ".to_string(),
        cell::CellState::Forbidden => "   ".to_string(),
    }
}

