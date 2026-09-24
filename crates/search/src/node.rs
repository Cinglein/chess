use board::{Board, ChessMove};

use crate::depth::Depth;
use crate::root_distance::RootDistance;
use crate::window::Window;

#[derive(Clone, Copy)]
pub(crate) struct Node<'position> {
    board: &'position Board,
    depth: Depth,
    distance: RootDistance,
    window: Window,
    hash_move: Option<ChessMove>,
}

impl<'position> Node<'position> {
    pub(crate) const fn new(
        board: &'position Board,
        depth: Depth,
        distance: RootDistance,
        window: Window,
    ) -> Self {
        Node {
            board,
            depth,
            distance,
            window,
            hash_move: None,
        }
    }

    pub(crate) const fn board(&self) -> &'position Board {
        self.board
    }

    pub(crate) const fn depth(&self) -> Depth {
        self.depth
    }

    pub(crate) const fn distance(&self) -> RootDistance {
        self.distance
    }

    pub(crate) const fn window(&self) -> Window {
        self.window
    }

    pub(crate) const fn hash_move(&self) -> Option<ChessMove> {
        self.hash_move
    }

    pub(crate) const fn remembering(self, hash_move: Option<ChessMove>) -> Self {
        Node { hash_move, ..self }
    }

    pub(crate) const fn at_depth(self, depth: Depth) -> Self {
        Node { depth, ..self }
    }
}
