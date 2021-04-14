#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (rank | file) < 0 || (rank | file) > 7 {
            None
        } else {
            Some(Self { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank || self.position.file == other.position.file {
            true
        } else {
            let distance_1 = self.position.file - other.position.file;
            let distance_2 = self.position.rank - other.position.rank;
            if distance_1.abs() == distance_2.abs() {
                true
            } else {
                false
            }
        }
    }
}
