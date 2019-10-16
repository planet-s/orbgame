use crate::{
    prelude::*,
    utils::{Map as MapValue, MapExt},
};

property!(
    Map(MapValue) : &str, 
    String
);

impl MapExt for Map {
    fn layer_count(&self) -> usize {
        self.0.layer_count()
    }

    fn row_count(&self) -> usize {
       self.0.row_count()
    }

    fn column_count(&self) -> usize {
       self.0.column_count()
    }

    fn tile_size(&self) -> u32 {
        self.0.tile_size()
    }

    fn get_tile(&self, layer: usize, row: usize, column: usize) -> i32 {
       self.0.get_tile(layer, row, column)
    }

    fn get_column(&self, x: f32) -> f32 {
        self.0.get_column(x)
    }

    fn get_row(&self, y: f32) -> f32 {
       self.0.get_row(y)
    }

    fn get_x(&self, column: f32) -> f32 {
       self.0.get_x(column)
    }

    fn get_y(&self, row: f32) -> f32 {
        self.0.get_y(row)
    }

    fn is_blocked(&self, column: usize, row: usize) -> bool {
       self.0.is_blocked(column, row)
    }

    fn set_tile(&mut self, layer: usize, column: usize, row: usize, tile: i32) {
       self.0.set_tile(layer, column, row, tile);
    }

    fn is_tile_blocked(&self, x: f32, y: f32) -> bool {
        self.0.is_tile_blocked(x, y)
    }
}