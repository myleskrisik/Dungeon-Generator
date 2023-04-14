use macroquad::prelude::{
   draw_rectangle,
   clear_background,
   next_frame,
   vec2,
   WHITE,
   BLACK,
   BLUE,
};
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, },
};

use macroquad::color::Color;

use rand::{
    seq::SliceRandom,
    thread_rng
};

const TILE_RECT_SIZE: f32 = 20.;
const TILE_GAP: f32 = 5.;
const DRAWING_START: (f32, f32) = (10., 10.);

enum TileType {
    Wall,
    Corner,
    Hall,
}

enum Orientation {
    Vertical,
    Horizontal
}

pub struct Tile {
    x: f32,
    y: f32,
    collapsed: bool,
    tile_type: Option<TileType>,
    orientation: Option<Orientation>,
    color: macroquad::color::Color,
}

impl Tile {
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, TILE_RECT_SIZE, TILE_RECT_SIZE, self.color);
    }

    pub fn change_color(&mut self, color: Color) {
        self.color = color;
    }
}


#[macroquad::main("Dungeon Generator")]
async fn main() {
    
    let mut dungeon_size = 0.;
    let mut dungeon = Vec::new(); 
    loop {
        clear_background(WHITE);

        widgets::Window::new(hash!(), vec2(40., 50.), vec2(300., 100.))
            .label("Dungeon Size")
            .ui(&mut *root_ui(), |ui| {
                ui.slider(hash!(), "[0 .. 50]", 0f32..50f32, &mut dungeon_size);
                if ui.button(None, "Generate") {
                   dungeon = populate_tiles(dungeon_size);
                   collapse(&mut dungeon);
                }            
            });
        dungeon_size = dungeon_size.round();

        for room in &dungeon {
//            if room.collapsed {
                room.draw();
 //           } 
        }
        next_frame().await
    }
}

fn populate_tiles(dungeon_size: f32) -> Vec<Tile> {
    let mut dungeon = Vec::new();
    for i in 0u32..dungeon_size as u32 {
        for j in 0u32..dungeon_size as u32 {
            let room_x = (i as f32 * (TILE_RECT_SIZE + TILE_GAP)) + DRAWING_START.0;
            let room_y = (j as f32 * (TILE_RECT_SIZE + TILE_GAP)) + DRAWING_START.1;
            dungeon.push(Tile {x: room_x, y: room_y, collapsed: false, tile_type: None, orientation: None, color: BLACK}); 
        }
    }
    dungeon
}

fn collapse(dungeon: &mut Vec<Tile>) {
    let base_room: Option<&mut Tile> = dungeon.choose_mut(&mut thread_rng());
}
