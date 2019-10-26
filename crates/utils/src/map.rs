use std::{fs::File, io::prelude::*};

use ron::de::from_str;
use serde_derive::Deserialize;

use orbtk::prelude::*;

#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct Layer {
    pub tiles: Vec<i32>,
}

impl Layer {
    pub fn push(&mut self, tile: i32) {
        self.tiles.push(tile);
    }

    pub fn set_tile(&mut self, index: usize, tile: i32) {
        if let Some(t) = self.tiles.get_mut(index) {
            *t = tile;
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct Map {
    pub layer_count: usize,
    pub row_count: usize,
    pub column_count: usize,
    pub tile_size: u32,
    pub blocked_tiles: Vec<i32>,
    pub layers: Vec<Layer>,
}

into_property_source!(Map: &str, String);

impl Map {
    pub fn layer_count(&self) -> usize {
        self.layer_count
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn column_count(&self) -> usize {
        self.column_count
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    pub fn get_tile(&self, layer: usize, row: usize, column: usize) -> i32 {
        if let Some(l) = self.layers.get(layer) {
            if let Some(t) = l.tiles.get(row * self.column_count + column) {
                return *t;
            }
        }
        -1
    }

    pub fn get_column(&self, x: f32) -> f32 {
        (x / self.tile_size as f32).floor()
    }

    pub fn get_row(&self, y: f32) -> f32 {
        (y / self.tile_size as f32).floor()
    }

    pub fn get_x(&self, column: f32) -> f32 {
        column * self.tile_size as f32
    }

    pub fn get_y(&self, row: f32) -> f32 {
        row * self.tile_size as f32
    }

    pub fn is_blocked(&self, column: usize, row: usize) -> bool {
        for l in &self.layers {
            if let Some(t) = l.tiles.get(row * self.column_count + column) {
                if self.blocked_tiles.contains(&t) {
                    return true;
                }
            }
        }

        false
    }

    pub fn set_tile(&mut self, layer: usize, column: usize, row: usize, tile: i32) {
        if let Some(layer) = self.layers.get_mut(layer) {
            layer.set_tile(row * self.column_count + column, tile);
        }
    }

    pub fn is_tile_blocked(&self, x: f32, y: f32) -> bool {
        let column = (x / self.tile_size as f32).floor() as usize;
        let row = (y / self.tile_size as f32).floor() as usize;

        self.is_blocked(column, row)
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        if let Ok(file) = &mut File::open(s) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let map: Map = match from_str(contents.as_str()) {
                Ok(x) => x,
                Err(e) => {
                    println!("Failed to load config: {:?}", e);

                    std::process::exit(1);
                }
            };

            return map;
        } else {
            panic!("Could not load file {}", s);
        }
    }
}

impl From<String> for Map {
    fn from(string: String) -> Self {
        Map::from(string.as_str())
    }
}
