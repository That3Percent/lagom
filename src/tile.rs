#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SYMBOL {
    FRIEND,
    COIN,
    PURPOSE(u8),
    ACHIEVEMENT,
    PARENT,
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Tile {
    pub top: Vec<SYMBOL>,
    pub bottom_left: Vec<SYMBOL>,
    pub bottom_right: Vec<SYMBOL>,
    pub center: Vec<SYMBOL>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TilePlacement {
    pub x: i32,
    pub y: i32,
    pub rotation: i32,
    pub tile: Tile,
}

impl TilePlacement {
    pub fn tile_is_flipped(&self) -> bool {
        (self.x + self.y).abs() % 2 == 1
    }

    pub fn covered_positions(&self) -> [(i32, i32); 4] {
        let x = self.x;
        let y = self.y;
        let flipped = self.tile_is_flipped();
        [
            (x, y),
            (x, y+ if flipped { 1 } else {-1}),
            (x+1, y),
            (x-1, y),
        ]
    }
}
