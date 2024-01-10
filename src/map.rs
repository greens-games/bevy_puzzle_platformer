use std::borrow::BorrowMut;
use std::fs;

use bevy::prelude::*;
use std::collections::HashMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap {
            ..Default::default()
        });
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TileType {
    WALL,
    POWER_UP,
    PLAYER
}

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType
}

#[derive(Resource)]
pub struct TileMap {
    pub map: Vec<Tile>
}

impl Default for TileMap {
    fn default() -> Self {
        load_level_1(Vec::new())
    }
}


fn load_level_1(mut tile_map: Vec<Tile>) -> TileMap {
    //read in map file
    let level_conents = fs::read_to_string("./src/level1.txt").expect("File not found for level");

    for (y, line) in level_conents.lines().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            //depending on letter we want to spawn something @(x*16, y*16)
            //create a new tile
            match letter {
                'p' =>  {
                    tile_map.insert(0, Tile {
                        x: x,
                        y: y,
                        tile_type: TileType::PLAYER
                    })

            },
                'f' => {
                    println!("Floor x: {}, Floor y: {}", x, y);
                    tile_map.insert(0, Tile {
                        x: x,
                        y: y,
                        tile_type: TileType::WALL
                    })

            },
                'w' => {
                    tile_map.insert(0, Tile {
                        x: x,
                        y: y,
                        tile_type: TileType::WALL
                    })

            },
                'E' => {
                    tile_map.insert(0, Tile {
                        x: x,
                        y: y,
                        tile_type: TileType::POWER_UP
                    })

            },
                'j' => {
                    tile_map.insert(0, Tile {
                        x: x,
                        y: y,
                        tile_type: TileType::POWER_UP
                    })

            },
                _ => {}
            }
            //add tile to list of Tiles for that Type
            // size of sprite is base_size*scale factor, scale_factor = Vec2::new(Window.resolution.width/256, Window.resolution.height/144) this keeps us in 16:9 aspect ratio
        } 

    }

    TileMap {
        map: tile_map
    }

    //parse map file
    //if char is found save x,y coords and type of cell
    //else is blank and continue
}