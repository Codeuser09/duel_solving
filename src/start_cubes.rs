use crate::cube;

pub struct StartCubes {
    pub five: [[i32; 4]; 2],
    pub one: [[i32; 4]; 2],
    pub two: [[i32; 4]; 2],
    pub six: [[i32; 4]; 2],
    pub king: [[i32; 4]; 2],
    pub zero: [[i32; 4]; 2],
}

impl StartCubes {
    // Function to initialize cube values and memoize them
    pub fn new() -> StartCubes {
        let five: [[i32; 4]; 2] = [[5, 3, 2, 4], [5, 1, 2, 6]];
        let one: [[i32; 4]; 2] = cube::roll(1, true, five);
        let two: [[i32; 4]; 2] = cube::roll(1, true, one);
        let six: [[i32; 4]; 2] = cube::roll(1, true, two);
        let king: [[i32; 4]; 2] = [[1, 1, 1, 1], [1, 1, 1, 1]];
        let zero: [[i32; 4]; 2] = [[0, 0, 0, 0], [0, 0, 0, 0]];

        StartCubes {
            five,
            one,
            two,
            six,
            king,
            zero,
        }
    }
}
