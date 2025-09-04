mod tile;
mod world_state;
mod renderer;
mod view;

use crate::{
    renderer::Textures, tile::{Tile, TilePlacement, SYMBOL}, world_state::WorldState
};
use macroquad::{prelude::*};

#[macroquad::main("Lagom")]
async fn main() {

    let one_coin = || vec![SYMBOL::COIN];
    let parent_tile = Tile {
        top: one_coin(),
        bottom_left: one_coin(),
        bottom_right: one_coin(),
        center: vec![SYMBOL::PARENT],
    };

    let textures = Textures::load().await;

   
    // Create the initial setup with two 'parent' tiles
    let mut world = WorldState {
        held_tile: None,
        tiles: vec![
            TilePlacement {
                x: -1,
                y: -7,
                rotation: 0,
                tile: parent_tile.clone(),
            },
            TilePlacement {
                x: 1,
                y: -7,
                rotation: 0,
                tile: parent_tile.clone(),
            },
        ],
        view: view::View::new(),
    };


    let mut full_screen = false;
    
    // Game loop
    loop {
        // Update
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            full_screen = !full_screen;
            set_fullscreen(full_screen);
        }
        world.view.update();
        world.update_held_tile();

        // Render
        clear_background(SKYBLUE);
        renderer::render(&world, &textures);

        // Next frame
        next_frame().await;
    }
}
