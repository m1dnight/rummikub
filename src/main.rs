use crate::mutations::place_tiles_best;
use crate::structures::{Board, Run};
use crate::structures::{Color, Tile};

mod mutations;
pub mod structures;

fn main() {
    let mut board = test_board();
    println!("{}", board.to_string());

    let tiles = vec![
        Tile {
            color: Color::Blue,
            value: 3,
        },
        Tile {
            color: Color::Blue,
            value: 4,
        },
        Tile {
            color: Color::Black,
            value: 1,
        },
        Tile {
            color: Color::Blue,
            value: 2,
        },
    ];

    match place_tiles_best(&board, &tiles) {
        Some((new_board, placed_tiles, unplaced_tiles)) => {
            println!("Found a solution");
            println!(
                "Tiles placed: {}",
                placed_tiles
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            println!(
                "Remaining tiles: {}",
                unplaced_tiles
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            println!("before:\n{}", board.to_string());
            board = new_board;

            println!("after:\n{}", board.to_string());
        }
        None => {
            println!("no solution found");
        }
    }
}

fn test_board() -> Board {
    let mut board = Board {
        groups: vec![],
        runs: vec![],
    };

    board.runs.push(Run {
        start: 2,
        end: 5,
        color: Color::Blue,
    });

    board.runs.push(Run {
        start: 6,
        end: 13,
        color: Color::Blue,
    });

    // board.runs.push(Run {
    //     start: 1,
    //     end: 3,
    //     color: Color::Orange,
    // });
    // board.groups.push(Group {
    //     value: 10,
    //     colors: vec![Color::Orange, Color::Blue, Color::Red],
    // });
    return board;
}
