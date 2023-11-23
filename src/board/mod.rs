mod constants;
mod utils;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    White = 0,
    Black = 1,
    Both = 2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rooke = 3,
    Queen = 4,
    King = 5,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Castling {
    WhiteKing = 1,
    WhiteQueen = 2,
    BlackKing = 4,
    BlackQueen = 8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board {
    pub bitboards: [u64; 12],
    pub turn: u64,
    pub castling: u8,
    pub en_passant: u64,
}

pub fn new_game() -> Board {
    let mut b = Board {
        bitboards: [0; 12],
        turn: 0,
        castling: 0,
        en_passant: 0,
    };
    b.reset_board();
    b
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboards: [0; 12],
            turn: 0,
            castling: 0,
            en_passant: 0,
        }
    }

    pub fn reset_board(&mut self) {
        self.bitboards = constants::STARTING_POSITIONS;
        self.turn = 0;
        self.castling = constants::CASTLING;
        self.en_passant = 0;
    }

    pub fn get_board(&self, piece: Piece, player: Player) -> u64 {
        match player {
            Player::Both => self.bitboards[piece as usize] + self.bitboards[(piece as usize) + 6],
            Player::White => self.bitboards[piece as usize],
            Player::Black => self.bitboards[(piece as usize) + 6],
        }
    }

    fn get_bitboard_from_pos(&self, pos: u64) -> Result<usize, String> {
        for i in 0..12 {
            if self.bitboards[i] & pos != 0 {
                return Ok(i);
            }
        }
        Err(format!("No piece at position {}", pos))
    }

    ///Makes a move on the board, this trusts that the move being made is valid
    pub fn move_piece(&mut self, from: u64, to: u64) -> Result<(), String> {
        if utils::only_one_bit_set(from) == false {}

        let moving_piece: usize = match self.get_bitboard_from_pos(from) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        self.bitboards[moving_piece] ^= from;
        self.bitboards[moving_piece] |= to;

        let attacked_piece: usize = match self.get_bitboard_from_pos(to) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        self.bitboards[attacked_piece] ^= to;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //Make sure that the board is set up correctly
    #[test]
    fn test_get_board() {
        let mut b = Board::new();
        b.reset_board();
        let types = [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rooke,
            Piece::Queen,
            Piece::King,
        ];
        for i in 0..types.len() {
            assert_eq!(
                b.get_board(types[i], Player::Both),
                constants::STARTING_POSITIONS[i] + constants::STARTING_POSITIONS[i + 6]
            );
        }
    }
}
