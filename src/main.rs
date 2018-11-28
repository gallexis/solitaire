#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_comparisons)]
#![allow(unused_imports)]

extern crate rand;

mod game;
mod board;
mod cell;
mod coord;
mod utils;
mod ui;

use coord::ForbiddenPoints;
use coord::Point;

fn main() {
    let forbidden_points: ForbiddenPoints = vec![Point { x: 0, y: 0 },
                                                 Point { x: 1, y: 0 },
                                                 Point { x: 5, y: 0 },
                                                 Point { x: 6, y: 0 },
                                                 Point { x: 0, y: 1 },
                                                 Point { x: 1, y: 1 },
                                                 Point { x: 5, y: 1 },
                                                 Point { x: 6, y: 1 },
                                                 Point { x: 0, y: 5 },
                                                 Point { x: 1, y: 5 },
                                                 Point { x: 5, y: 5 },
                                                 Point { x: 6, y: 5 },
                                                 Point { x: 0, y: 6 },
                                                 Point { x: 1, y: 6 },
                                                 Point { x: 5, y: 6 },
                                                 Point { x: 6, y: 6 },
    ];

    let board = board::Board::new(7, 7, Point { x: 2, y:0}, forbidden_points);
    let mut g = game::Game::new(board.clone());

    for i in 1.. {
        g.play_again(board.clone());
        g.start();
        if g.finished{
            g.display();
            break
        }
    }
}
