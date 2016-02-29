use std::cmp::Ordering;

use args::{Coords, Direction};
use args::Direction::*;

use self::Movement::*;

/// Represents a manner in which the cursor can be moved.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Movement {
    Position(Coords),
    To(Direction, u32, bool),
    ToEdge(Direction),
    IndexTo(Direction, u32),
    /// Arguments:
    /// * Direction of the tab character.
    /// * Number of tab stops "long" the movement should be.
    /// * Whether or not the movement should wrap when it reaches the end of the screen.
    Tab(Direction, u32, bool ),
    Column(u32),
    Row(u32),
    PreviousLine(u32),
    NextLine(u32),
    ToBeginning,
    ToEnd,
}

impl Movement {

    /// Returns the direction the cursor would travel in on taking this movement.
    pub fn direction(&self, cursor: Coords) -> Direction {
        match *self {
            To(d, _, _) | ToEdge(d) | IndexTo(d, _) | Tab(d, _, _)  => d,
            ToBeginning                                             => Left,
            ToEnd                                                   => Right,
            PreviousLine(_)                                         => Up,
            NextLine(_)                                             => Down,
            Column(n) if n < cursor.x                               => Left,
            Column(_)                                               => Right,
            Row(n) if n < cursor.y                                  => Up,
            Row(_)                                                  => Down,
            Position(coords)                                        => {
                match coords.y.cmp(&cursor.y) {
                    Ordering::Less                                  => Left,
                    Ordering::Equal if coords.x < cursor.x          => Left,
                    Ordering::Equal                                 => Right,
                    Ordering::Greater                               => Right,
                }
            }
        }
    }

    /// Returns true if this motion can cause the screen to scroll.
    pub fn scrolls(&self) -> bool {
        match *self {
            IndexTo(..) | PreviousLine(_) | NextLine(_) => true,
           _                                            => false,
        }
    }

}
