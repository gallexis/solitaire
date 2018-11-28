use board::Board;
use board::EdibleNeighbors;

use ui::board::display_board;

use coord::Point;

pub struct Game {
    pub finished: bool,
    pub board: Board,
    pub board_history: Vec<Board>,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game {
            board,
            finished: false,
            board_history: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.board.init_board();
        self.board_history.push(self.board.clone());

        while self.board.random_eat() {
            self.board_history.push(self.board.clone());
        }

        if self.board.remaining_cells <= 1 {
            self.finished = true;
        }
    }

    pub fn display(&self) {
        for b in self.board_history.iter() {
            println!("{}", display_board(b));
        }
    }

    pub fn play_again(&mut self, board: Board) {
        *self = Game::new(board);
        self.start();
    }
}
