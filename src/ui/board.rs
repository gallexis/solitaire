use board;
use super::cell::display_cell;

use std::fmt::Write as FmtWrite;


pub fn display_board(board: &board::Board) -> String {
    let mut row = 0;
    let mut buffer = String::new();
    let width = board.width;
    let mut cell_shape: String;

    writeln!(&mut buffer, "\n- - - - - - - - - - - - -").unwrap();

    for i in 0..width {
        write!(&mut buffer, " {} ", i).unwrap();
    }
    writeln!(&mut buffer, "\n________________________").unwrap();

    for (i, cell) in board.cells.iter().enumerate() {
        cell_shape =  display_cell(cell);

        if ((i + 1) % width as usize) == 0 {
            writeln!(&mut buffer, "{} |{}", cell_shape, row);
            row += 1;
        } else {
            write!(&mut buffer, "{}", cell_shape, );
        }
    }
    writeln!(&mut buffer, "________________________\n\n").unwrap();

    buffer
}