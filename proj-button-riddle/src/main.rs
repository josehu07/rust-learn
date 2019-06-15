use button_riddle::{button_puzzle, button_backpack};


fn main() {

    // Puzzle.
    let puzzle = button_puzzle::ButtonPuzzle::new([9, 4, 7,
                                                   8, 1, 6,
                                                   5, 3, 2]);
    println!("{:#?}", puzzle.solve().unwrap());

    // Backpack.
    let config = button_backpack::ButtonBackpack::new(99, (4, 5, 7));
    println!("{:#?}", config.solve().unwrap());
}
