// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Robot {
    d: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        match self.d {
            Direction::West => Robot {
                d: Direction::North,
                ..self
            },
            Direction::North => Robot {
                d: Direction::East,
                ..self
            },
            Direction::South => Robot {
                d: Direction::West,
                ..self
            },
            Direction::East => Robot {
                d: Direction::South,
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::West => Robot {
                d: Direction::South,
                ..self
            },
            Direction::North => Robot {
                d: Direction::West,
                ..self
            },
            Direction::South => Robot {
                d: Direction::East,
                ..self
            },
            Direction::East => Robot {
                d: Direction::North,
                ..self
            },
        }
    }

    pub fn advance(self) -> Self {
        match self.d {
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        // Recursive solution. I'm surprised this worked.
        let length = instructions.len();
        if length < 1 {
            // Base case. Are all instructions consumed?
            Self { ..self } // Then we're done! Return this as the result.
        } else {
            // Otherwise
            let mut tmp = self; // make a temporary mutable variable and store a copy of self
            match instructions.chars().next().unwrap() { // take a look at the next instruction
                'A' => tmp = self.advance(), // Save the resulting state in the variable
                'L' => tmp = self.turn_left(),
                'R' => tmp = self.turn_right(),
                _ => panic!("Aku suka makan daging! .... Oh, you didn't understand that? Well, I didn't understand your instruction either.")
};
            tmp.instructions(instructions.get(1..length).unwrap()) // Recursive method call, take away one instruction unit from the start.
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
