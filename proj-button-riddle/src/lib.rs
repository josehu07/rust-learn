extern crate queues;
extern crate strum;
#[macro_use]
extern crate strum_macros;


pub mod button_puzzle {

    use std::collections::HashSet;
    use queues::*;
    use strum::IntoEnumIterator;

    #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumIter)]
    pub enum ButtonOp { RotateWise, RotateBack, ShuffleUp }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct ButtonPuzzle {
        state : [u8 ; 9],
        trace : Vec<ButtonOp>,
    }

    impl ButtonPuzzle {

        pub fn new(state : [u8 ; 9]) -> ButtonPuzzle {
            ButtonPuzzle {
                state,
                trace : vec![],
            }
        }

        fn operate(&self , op : ButtonOp) -> ButtonPuzzle {
            let [lt, ct, rt, lc, cc, rc, lb, cb, rb] = self.state;
            let mut new_trace = self.trace.clone();
            new_trace.push(op);
            match op {
                ButtonOp::RotateWise => {
                    ButtonPuzzle {
                        state : [lc, lt, ct, lb, cc, rt, cb, rb, rc],
                        trace : new_trace,
                    }
                },
                ButtonOp::RotateBack => {
                    ButtonPuzzle {
                        state : [ct, rt, rc, lt, cc, rb, lc, lb, cb],
                        trace : new_trace,
                    }
                },
                ButtonOp::ShuffleUp => {
                    ButtonPuzzle {
                        state : [lc, cc, rc, lb, cb, rb, lt, ct, rt],
                        trace : new_trace,
                    }
                }
            }
        }

        fn is_solution(&self) -> bool {
            self.state == [1, 2, 3, 4, 5, 6, 7, 8, 9]
        }

        pub fn solve(&self) -> Option<Vec<ButtonOp>> {

            // Initialize visit queue and puzzles hash table.
            let mut visit_queue = queue![self.clone()];
            let puzzles_met : HashSet<ButtonPuzzle> = HashSet::new();

            // Loop to fetch the next puzzle to visit.
            while let Ok(puzzle) = visit_queue.remove() {

                // If is the solution, return its trace.
                if puzzle.is_solution() {
                    return Some(puzzle.trace)
                }

                // Else, add its non-visited children into the queue.
                for op in ButtonOp::iter() {
                    let new_puzzle = puzzle.operate(op);
                    if !puzzles_met.contains(&new_puzzle) {
                        visit_queue.add(new_puzzle).unwrap();
                    }
                }
            }

            // Return None on failure.
            None
        }
    }
}


pub mod button_backpack {

    use std::collections::HashSet;
    use queues::*;
    use strum::IntoEnumIterator;

    #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumIter)]
    pub enum ButtonOp { Yellow, Red, Green }

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    pub struct ButtonBackpack {
        meter : i32,
        items : (i32, i32, i32),
        trace : Vec<ButtonOp>,
    }

    impl ButtonBackpack {

        pub fn new(meter : i32, items : (i32, i32, i32)) -> ButtonBackpack {
            ButtonBackpack {
                meter,
                items,
                trace : vec![],
            }
        }

        fn operate(&self, op : ButtonOp) -> ButtonBackpack {
            let old_meter = self.meter;
            let (v1, v2, v3) = self.items;
            let mut new_trace = self.trace.clone();
            new_trace.push(op);
            match op {
                ButtonOp::Yellow => {
                    ButtonBackpack {
                        meter : old_meter - v1,
                        items : (v1, v2 + v1, v3 + v1),
                        trace : new_trace,
                    }
                },
                ButtonOp::Red => {
                    ButtonBackpack {
                        meter : old_meter - v2,
                        items : (v1 + v2, v2, v3 + v2),
                        trace : new_trace,
                    }
                },
                ButtonOp::Green => {
                    ButtonBackpack {
                        meter : old_meter - v3,
                        items : (v1 + v3, v2 + v3, v3),
                        trace : new_trace,
                    }
                }
            }
        }

        fn is_solution(&self) -> bool {
            self.meter == 0
        }

        fn is_valid(&self) -> bool {
            self.meter >= 0
        }

        pub fn solve(&self) -> Option<Vec<ButtonOp>> {

            // Initialize visit queue and puzzles hash table.
            let mut visit_queue = queue![self.clone()];
            let configs_met : HashSet<ButtonBackpack> = HashSet::new();

            // Loop to fetch the next config to visit.
            while let Ok(config) = visit_queue.remove() {

                // If is the solution, return its trace.
                if config.is_solution() {
                    return Some(config.trace)
                }

                // Else, add its non-visited children into the queue.
                for op in ButtonOp::iter() {
                    let new_config = config.operate(op);
                    if !configs_met.contains(&new_config) && new_config.is_valid() {
                        visit_queue.add(new_config).unwrap();
                    }
                }
            }

            // Return None on failure.
            None
        }
    }
}
