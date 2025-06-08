use crate::structures::{Board, Run, Tile};
use itertools::Itertools;

/// Extend a run by adding a tile at the end.
///
/// Attempts to add a tile to the end of an existing run. The tile must:
/// - Have the same color as the run
/// - Have a value that is exactly one greater than the run's end value
///
/// # Arguments
/// * `run` - The existing run to extend
/// * `tile` - The tile to add at the end
///
/// # Returns
/// * `Some(Run)` - A new run with the tile added if extension is valid
/// * `None` - If the tile cannot be added (wrong color or non-consecutive value)
///
/// # Example
/// ```
/// // Run: Blue 3-4-5 (end=5), Tile: Blue 6
/// // Returns: Some(Run { start: 3, end: 6, color: Blue })
/// ```
fn extend_run_tail(run: &Run, tile: &Tile) -> Option<Run> {
    if run.color == tile.color && run.end == tile.value - 1 {
        let new_run = Some(Run {
            start: run.start,
            end: tile.value,
            color: run.color,
        });
        new_run
    } else {
        None
    }
}

/// Extend a run by adding a tile at the beginning.
///
/// Attempts to add a tile to the beginning of an existing run. The tile must:
/// - Have the same color as the run
/// - Have a value that is exactly one less than the run's start value
///
/// # Arguments
/// * `run` - The existing run to extend
/// * `tile` - The tile to add at the beginning
///
/// # Returns
/// * `Some(Run)` - A new run with the tile added if extension is valid
/// * `None` - If the tile cannot be added (wrong color or non-consecutive value)
///
/// # Example
/// ```
/// // Run: Blue 3-4-5 (start=3), Tile: Blue 2
/// // Returns: Some(Run { start: 2, end: 5, color: Blue })
/// ```
fn extend_run_head(run: &Run, tile: &Tile) -> Option<Run> {
    if run.color == tile.color && run.start == tile.value + 1 {
        let new_run = Some(Run {
            start: tile.value,
            end: run.end,
            color: run.color,
        });
        new_run
    } else {
        None
    }
}

/// Generate all possible ways to split a run into multiple runs.
///
/// Returns all possible ways to break a run into separate, non-overlapping runs.
/// For a run 1-3, this returns: [[1], [2,3]], [[1,2], [3]], [[1], [2], [3]].
///
/// # Arguments
/// * `run` - The run to split into multiple runs
///
/// # Returns
/// * `Vec<Vec<Run>>` - All possible ways to split the run, where each inner Vec
///   contains the runs for that particular split
///
/// # Example
/// ```
/// // Run: Blue 1-3 (values 1, 2, 3)
/// // Returns: [
/// //   [Run(1,1), Run(2,3)],  // Split after 1
/// //   [Run(1,2), Run(3,3)],  // Split after 2  
/// //   [Run(1,1), Run(2,2), Run(3,3)]  // Split after 1 and 2
/// // ]
/// ```
fn split_run(run: &Run) -> Vec<Vec<Run>> {
    let mut all_splits = Vec::new();
    let run_length = run.end - run.start + 1;

    // Generate all possible split points (between tiles)
    // For run 1-3, split points are after 1 and after 2
    for split_mask in 1..(1 << (run_length - 1)) {
        let mut current_split = Vec::new();
        let mut current_start = run.start;

        for i in 0..(run_length - 1) {
            if (split_mask & (1 << i)) != 0 {
                // There's a split after position current_start + i
                let current_end = run.start + i;
                current_split.push(Run {
                    start: current_start,
                    end: current_end,
                    color: run.color,
                });
                current_start = current_end + 1;
            }
        }

        // Add the final segment
        current_split.push(Run {
            start: current_start,
            end: run.end,
            color: run.color,
        });

        all_splits.push(current_split);
    }

    all_splits
}

/// Compress adjacent runs of the same color into larger runs.
///
/// Takes a collection of runs and merges any that are adjacent and of the same color.
/// For example, [Blue 1-2, Blue 3-4] becomes [Blue 1-4].
///
/// # Arguments
/// * `runs` - Slice of runs to compress
///
/// # Returns
/// * `Vec<Run>` - Compressed runs with adjacent runs merged
///
/// # Example
/// ```
/// // Input: [Blue 1-2, Blue 3-4, Red 5-6]
/// // Output: [Blue 1-4, Red 5-6]
/// ```
fn compress_runs(runs: &[Run]) -> Vec<Run> {
    if runs.is_empty() {
        return vec![];
    }

    let mut compressed = Vec::new();
    let mut sorted_runs = runs.to_vec();

    // Sort runs by color first, then by start position
    sorted_runs.sort_by(|a, b| match a.color as u8 {
        ac => match b.color as u8 {
            bc => {
                if ac == bc {
                    a.start.cmp(&b.start)
                } else {
                    ac.cmp(&bc)
                }
            }
        },
    });

    let mut current_run = sorted_runs[0].clone();

    for run in sorted_runs.iter().skip(1) {
        // Check if this run can be merged with the current run
        if run.color == current_run.color && run.start == current_run.end + 1 {
            // Extend the current run
            current_run.end = run.end;
        } else {
            // Can't merge, add current run to result and start a new one
            compressed.push(current_run);
            current_run = run.clone();
        }
    }

    // Don't forget the last run
    compressed.push(current_run);

    compressed
}

/// Extend a run by adding a tile at either end.
///
/// Attempts to extend a run by first trying to add the tile at the beginning,
/// and if that fails, trying to add it at the end.
///
/// # Arguments
/// * `run` - The existing run to extend
/// * `tile` - The tile to add
///
/// # Returns
/// * `Some(Run)` - A new run with the tile added if extension is possible
/// * `None` - If the tile cannot be added to either end
///
/// # Example
/// ```
/// // Run: Blue 3-4-5, Tile: Blue 2 -> extends at head
/// // Run: Blue 3-4-5, Tile: Blue 6 -> extends at tail
/// // Run: Blue 3-4-5, Tile: Blue 8 -> returns None
/// ```
fn extend_run(run: &Run, tile: &Tile) -> Option<Run> {
    match extend_run_head(run, tile) {
        Some(run) => Some(run),
        None => extend_run_tail(run, tile),
    }
}

/// Attempt to place a single tile on the board.
///
/// Currently only tries to extend the first run on the board. The tile must
/// be able to extend the run at either the beginning or end.
///
/// # Arguments
/// * `board` - The current game board
/// * `tile` - The tile to place
///
/// # Returns
/// * `Some(Board)` - A new board with the tile placed if successful
/// * `None` - If the tile cannot be placed
///
pub fn place_tile(board: &Board, tile: &Tile) -> Option<Board> {
    let mut new_board = board.clone();

    // Try to extend each run on the board
    for (index, run) in new_board.runs.iter().enumerate() {
        if let Some(extended_run) = extend_run(run, tile) {
            new_board.runs[index] = extended_run;
            println!("placed {}", tile);
            println!("{}", new_board.to_string());
            return Some(new_board);
        }
    }

    // If no extension worked, try splitting runs and placing tile in the splits
    for (run_index, run) in board.runs.iter().enumerate() {
        let all_splits = split_run(run);

        for split_combination in all_splits {
            // Try to extend each run in this split combination with the tile
            for (split_index, split_run) in split_combination.iter().enumerate() {
                if let Some(extended_split) = extend_run(split_run, tile) {
                    // Create a new board with this split combination, replacing the original run
                    let mut split_board = board.clone();

                    // Remove the original run
                    split_board.runs.remove(run_index);

                    // Add all the split runs, with one extended
                    for (i, split_run_copy) in split_combination.iter().enumerate() {
                        if i == split_index {
                            split_board
                                .runs
                                .insert(run_index + i, extended_split.clone());
                        } else {
                            split_board
                                .runs
                                .insert(run_index + i, split_run_copy.clone());
                        }
                    }
                    println!("split and placed {}", tile);
                    println!("{}", split_board.to_string());
                    return Some(split_board);
                }
            }
        }
    }

    None
}

/// Recursively place multiple tiles on the board.
///
/// Attempts to place tiles one by one in the given order. For each tile that
/// can be placed, it recursively tries to place the remaining tiles on the
/// updated board. This ensures maximum tile placement.
///
/// The result is the modified board, and the list of tiles that are placed.
///
/// # Arguments
/// * `board` - The current game board
/// * `tiles` - Vector of tile references to place
///
/// # Returns
/// * `(Board, Vec<&Tile>)` - Tuple containing:
///   - The updated board after placing tiles
///   - Vector of tiles that were successfully placed
///
/// # Example
/// ```
/// // Given tiles [Blue 6, Blue 7, Red 5] and a board with Blue 4-5 run
/// // Might place Blue 6, Blue 7 and return (updated_board, [Blue 6, Blue 7])
/// ```
pub fn place_tiles(board: &Board, tiles: &[Tile]) -> (Board, Vec<Tile>) {
    let mut modified_board = board.clone();
    let mut placed_tiles: Vec<Tile> = vec![];

    for (index, tile) in tiles.iter().enumerate() {
        // try and place the tile, and if it succeeds, move on to the next
        match place_tile(&modified_board, tile) {
            Some(new_board) => {
                let remaining_tiles = &tiles[index + 1..];

                // add the tile we just placed
                placed_tiles.push(*tile);

                let (final_board, mut sub_placed_tiles) = place_tiles(&new_board, remaining_tiles);

                placed_tiles.append(&mut sub_placed_tiles);
                modified_board = final_board;
                break; // Found a placement, continue with remaining tiles
            }
            None => {
                // Continue to next tile if this one can't be placed
            }
        }
    }
    (modified_board, placed_tiles)
}

/// Find the optimal tile placement by trying all permutations.
///
/// Attempts to place tiles in all possible orders and returns the arrangement
/// that places the maximum number of tiles. This brute-force approach ensures
/// the best possible outcome but can be expensive for large tile sets.
///
/// # Arguments
/// * `board` - The current game board
/// * `tiles` - Slice of tiles to place optimally
///
/// # Returns
/// * `Option<(Board, Vec<Tile>, Vec<Tile>)>` - Option containing:
///   - The board after optimal tile placement
///   - Vector of tiles that were successfully placed (in placement order)
///   - Vector of tiles that could not be placed
///
/// # Performance
/// Time complexity is O(n! × placement_cost) where n is the number of tiles.
/// Use with caution for large tile sets.
///
/// # Example
/// ```
/// // Given tiles [Blue 6, Orange 4, Blue 1] and a board with runs Blue 2-5, Orange 1-3
/// // Might try different orders and find Blue 1, Blue 6, Orange 4 places optimally
/// // Returns (updated_board, [Blue 1, Blue 6, Orange 4])
/// ```
pub fn place_tiles_best(board: &Board, tiles: &[Tile]) -> Option<(Board, Vec<Tile>, Vec<Tile>)> {
    let mut best_board = board.clone();
    let mut best_tiles = vec![];

    for perm in tiles.iter().permutations(tiles.len()).unique() {
        let perm_tiles: Vec<Tile> = perm.into_iter().map(|&tile| tile).collect();
        let (attempt_board, placed_tiles) = place_tiles(board, &perm_tiles);

        // check if the attempted placement is valid
        if attempt_board.valid() {
            if placed_tiles.iter().count() > best_tiles.len() {
                best_board = attempt_board;
                best_tiles = placed_tiles;
            }
            if best_tiles.iter().count() == tiles.len() {
                // All tiles placed, compress runs before returning
                println!("before compress:\n{}", best_board.to_string());
                best_board.runs = compress_runs(&best_board.runs);
                println!("after compress:\n{}", best_board.to_string());
                return Some((best_board, best_tiles, vec![]));
            }
        }
    }

    if !best_tiles.is_empty() {
        // Some tiles were placed, calculate unplaced tiles and compress runs
        println!("before compress:\n{}", best_board.to_string());
        best_board.runs = compress_runs(&best_board.runs);
        let unplaced_tiles: Vec<Tile> = tiles
            .iter()
            .filter(|tile| !best_tiles.contains(tile))
            .cloned()
            .collect();
        println!("after compress:\n{}", best_board.to_string());
        Some((best_board, best_tiles, unplaced_tiles))
    } else {
        None
    }
}
