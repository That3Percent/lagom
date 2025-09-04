use core::f32;

use crate::{tile::{TilePlacement, SYMBOL}, view::View, world_state::WorldState};
use macroquad::prelude::*;

pub struct Textures {
    triangle: Texture2D,
    coin: Texture2D,
    purpose: Texture2D,
    friend: Texture2D,
    achievement: Texture2D,
    deck: Texture2D,
    parent: Texture2D,
}

pub const TILE_SIZE: Vec2 = Vec2 { x: 95.9 * 2.0, y: 82.5 * 2.0 };

pub const TRIANGLE_SIZE: Vec2 = Vec2 { x: TILE_SIZE.x / 4., y: TILE_SIZE.y / 2. };

pub fn mouse_to_triangle_index(view: &View) -> (i32, i32) {
    // The center of the screen is the target.
    let mut x = mouse_position().0 * 2. - screen_width() + view.x * view.scale;
    let mut y = mouse_position().1 * 2. - screen_height() + view.y * view.scale;

    
    // Divide by triangle size to normalize
    x /= TRIANGLE_SIZE.x * view.scale;
    y /= TRIANGLE_SIZE.y * view.scale;

    // Move to centroid (sort of)
    y += 0.5;
    
    let y = y.round() as i32;
    let x = x.round() as i32;
    
    (x, y)
}

// In world space
fn centroid(x: i32, y: i32) -> Vec2 {
    let is_flipped = (x + y).abs() % 2 == 1;
    let y_offset = if is_flipped { 1./3. } else { 2./3. };
    Vec2 {
        x: x as f32 * TRIANGLE_SIZE.x,
        y: ((y as f32) - y_offset) * TRIANGLE_SIZE.y,
    }
}

// Triangle is drawn around centroid.
fn draw_tile(placement: &TilePlacement, textures: &Textures, depth: f32, is_deck: bool) {
    let flipped = placement.tile_is_flipped();

    let texture = if is_deck { &textures.deck } else { &textures.triangle };

    let centroid = centroid(placement.x, placement.y);

    let x = centroid.x - TILE_SIZE.x / 2.;
    let y = centroid.y - TRIANGLE_SIZE.y * 4./3.;

    let rotation = (120.0 * placement.rotation as f32) + if flipped { 60.0 } else { 0.0 };
    let rotation = rotation * f32::consts::PI / 180.;
    
    // pivot is in world space (docs say screenspace, but it appears to be unnafected by view scale)
    let pivot = Some(Vec2 {
        x: centroid.x,
        y: centroid.y,
    });

    let params = DrawTextureParams {
            dest_size: Some(TILE_SIZE),
            rotation,
            pivot,
            ..Default::default()
        };
    let depth = 0.3 + (depth * 0.7);
    let color = Color::new(depth, depth, depth, 1.0);
    draw_texture_ex(texture, x, y, color, params);

    let draw_symbol = |symbol, vertical_offset, scale_offset, rotation_offset| {
        let symbol_size = Vec2 {
            x: scale_offset * TRIANGLE_SIZE.x,
            y: scale_offset* TRIANGLE_SIZE.x,
        };
        let x = centroid.x - symbol_size.x / 2.;
        let y = centroid.y - symbol_size.y / 2. - vertical_offset;
        let rotation = rotation + rotation_offset;
        let params = DrawTextureParams {
            dest_size: Some(symbol_size),
            rotation,
            pivot,
            ..Default::default()
        };

        let texture = match symbol {
            SYMBOL::ACHIEVEMENT => &textures.achievement,
            SYMBOL::COIN => &textures.coin,
            SYMBOL::FRIEND => &textures.friend,
            SYMBOL::PARENT => &textures.parent,
            SYMBOL::PURPOSE(_) => &textures.purpose,
        };
        draw_texture_ex(texture, x, y, WHITE, params);
    };

    // Draw symbols. Code so ugly!
    let mut center_offset = -1.;
    for center in &placement.tile.center {
        if placement.tile.center.len() == 2 {
            draw_symbol(*center, center_offset * TRIANGLE_SIZE.y/6., 0.5, 0.);
            center_offset += 2.;
        } else {
            draw_symbol(*center, 0., 1., 0.);
        }
    }
    let corners = [
        &placement.tile.top,
        &placement.tile.bottom_right,
        &placement.tile.bottom_left
    ];
    for (i, v) in corners.iter().enumerate() {
        let scale_offset = if v.len() == 2 { 0.5 } else { 1. };
        let rotation_offset = i as f32 * 120. * f32::consts::PI / 180.;
        let mut vertical_offset = 0.;
        if v.len() == 2 {
            vertical_offset -= TRIANGLE_SIZE.y/6.;
        }
        
        for symbol in *v {
            draw_symbol(*symbol, vertical_offset + TRIANGLE_SIZE.y * 2. / 3., scale_offset, rotation_offset);
            vertical_offset += TRIANGLE_SIZE.y/3.;
        }
    }
}

impl Textures {
    pub async fn load() -> Self {
        macro_rules! tex {($p:expr) => {
            load_texture(&format!("assets/{}.png", $p)).await.unwrap()
        };}

        set_default_filter_mode(FilterMode::Linear);

        let textures = Self {
            triangle: tex!("sectored-triangle-with-parchment"),
            coin: tex!("wc-wealth"),
            purpose: tex!("wc-purpose"),
            friend: tex!("wc-relationships"),
            achievement: tex!("wc-accomplishment"),
            deck: tex!("triangle-card-back"),
            parent: tex!("wc-lose-health"),
        };
        
        build_textures_atlas();

        textures
    }
}

pub fn render(world: &WorldState, textures: &Textures) {
    // Hack to draw deck by pretending it's like a tile.
    let deck_placement = TilePlacement {
        x: 0, y: 0, rotation: 0, tile: Default::default(),
    };
    self::draw_tile(&deck_placement, textures, 1.0, true);

    let held_tile = world.held_tile.as_ref();

    let count = (world.tiles.len() + if held_tile.is_some() { 1 } else { 0 }) as f32;

    let mut world_tiles = world.tiles.iter();

    let mut i = 0;
    let mut next_depth = move || {
        i += 1;
        i as f32 / count
    };

    let mut draw_tile = move |p: &TilePlacement| {
        draw_tile(p, textures, next_depth(), false);
    };

    if let Some(held_tile) = held_tile {
        for _ in 0..held_tile.insertion {
            if let Some(world_tile) = world_tiles.next() {
                draw_tile(&world_tile);
            }
        }
        let position = mouse_to_triangle_index(&world.view);
        let placement = TilePlacement {
            rotation: held_tile.rotation,
            x: position.0,
            y: position.1,
            tile: held_tile.tile.clone(),
        };
        draw_tile(&placement);
    }
    for tile in world_tiles {
        draw_tile(&tile);
    }
}