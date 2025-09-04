use crate::tile::{Tile, TilePlacement};
use crate::view::View;
use macroquad::prelude::*;
use ::rand::Rng;
use crate::renderer::mouse_to_triangle_index;
use crate::tile::SYMBOL;

pub struct HeldTile {
    pub insertion: usize,
    pub tile: Tile,
    pub rotation: i32,
}

pub struct WorldState {
    pub tiles: Vec<TilePlacement>,
    pub held_tile: Option<HeldTile>,
    pub view: View, 
}

fn random_tile() -> Tile {

    let mut rng = ::rand::rng();
    let mut tile = Tile { top: vec![], bottom_left: vec![], bottom_right: vec![], center: vec![] };
    for _ in 0..5 {
        loop {
            let symbol = match rng.random_range(0..4) {
                0 => SYMBOL::FRIEND,
                1 => SYMBOL::COIN,
                2 => SYMBOL::PURPOSE(0),
                3 => SYMBOL::ACHIEVEMENT,
                _ => unreachable!(),
            };

            let slot = match rng.random_range(0..4) {
                0 => &mut tile.center,
                1 => &mut tile.top,
                2 => &mut tile.bottom_right,
                3 => &mut tile.bottom_left,
                _ => unreachable!(),
            };


            // Max 2 symbols per slot
            if slot.len() < 2 {
                //if symbol != SYMBOL::COIN || !slot.contains(&SYMBOL::COIN) {
                    slot.push(symbol);
                    break;
                //}
            }
        }
    }

    tile
}



impl WorldState {
    pub fn update_held_tile(&mut self) {

        if is_key_pressed(KeyCode::Delete) || is_key_pressed(KeyCode::Backspace) {
            self.held_tile = None;
        }

        // Drop/pickup tiles
        if is_mouse_button_pressed(MouseButton::Left) {
            let position = mouse_to_triangle_index(&self.view);

            // TODO: Disallow placing over the deck

            let held = self.held_tile.take();
            if let Some(held) = held {
                let tile = held.tile;
                
                let placement = TilePlacement {
                    rotation: held.rotation,
                    x: position.0,
                    y: position.1,
                    tile,
                };
                self.tiles.insert(held.insertion, placement);
            } else {

                // Did they click on a tile
                let mut clicked_tile = None;
                for i in 0..self.tiles.len() {
                    if self.tiles[i].covered_positions().contains(&position) {
                        clicked_tile = Some(i);
                    }
                }
                if let Some(clicked_tile) = clicked_tile {
                    let clicked = self.tiles.remove(clicked_tile);
                    self.held_tile = Some(HeldTile { insertion: clicked_tile, tile: clicked.tile, rotation: clicked.rotation })
                } else if [(0, 0), (1, 0), (-1, 0), (0, -1)].contains(&position) {
                    // If they click on the deck, add a tile to hand.            
                    // If there is no held tile, and they clicked on the deck, draw a tile.
                    self.held_tile = Some(HeldTile { insertion: self.tiles.len(), tile: random_tile(), rotation: 0 });
                }
            }
        }

        if let Some(held) = &mut self.held_tile {
            // E: Rotate clockwise
            if is_key_pressed(KeyCode::E) {
                held.rotation = match held.rotation {
                    0 => 1,
                    1 => 2,
                    2 => 0,
                    _ => unreachable!()
                }
            }
            // Q: Rotate counter-clockwise
            if is_key_pressed(KeyCode::Q) {
                held.rotation = match held.rotation {
                    0 => 2,
                    1 => 0,
                    2 => 1,
                    _ => unreachable!()
                }
            }
            // W: Move toward top
            if is_key_pressed(KeyCode::W) {
                if held.insertion < self.tiles.len() {
                    held.insertion += 1;
                }
            }
            // S: Move toward bottom
            if is_key_pressed(KeyCode::S) {
                if held.insertion > 0 {
                    held.insertion -= 1;
                }
            }
        }
    }
}