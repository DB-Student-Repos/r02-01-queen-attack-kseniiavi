#[derive(Debug)]
pub struct ChessPosition{
    rank: i32, file: i32
}


#[derive(Debug)]
pub struct Queen{
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..=7).contains(&rank) && (0..=7).contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }


        /*unimplemented!(
            "Construct a ChessPosition struct, given the following rank, file: ({rank}, {file}). If the position is invalid return None."
        );*/
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {position}

        //unimplemented!("Given the chess position {position:?}, construct a Queen struct.");
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank || self.position.file == other.position.file || (self.position.rank - other.position.rank).abs() == (self.position.file - other.position.file).abs()

        //unimplemented!("Determine if this Queen can attack the other Queen {other:?}");
    }
}
