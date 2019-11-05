use std::f32;

use crate::{
    prelude::*,
    render::Image,
    utils::{Camera, Map, Point, Rectangle},
};

pub struct TileMapRenderObject;

impl Into<Box<dyn RenderObject>> for TileMapRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for TileMapRenderObject {
    fn render_self(&self, context: &mut Context<'_>, _: &Point) {
        let (bounds, camera, map, mut image) = {
            let widget = context.widget();
            (
                widget.clone::<Rectangle>("bounds"),
                widget.clone::<Camera>("camera"),
                widget.clone::<Map>("map"),
                widget.try_clone::<Image>("image"),
            )
        };

        if let Some(image) = &mut image {
            // draw the tile map

            let tile_size = map.tile_size;

            let start_column = (camera.x() as f32 / tile_size as f32).floor() as usize;
            let end_column =
                start_column + (camera.width() as f32 / tile_size as f32).ceil() as usize;
            let start_row = (camera.y() as f32 / tile_size as f32).floor() as usize;
            let end_row = start_row + (camera.height() as f32 / tile_size as f32).ceil() as usize;
            let offset_x = (start_column as f32)
                .mul_add(tile_size as f32, bounds.x as f32 + -camera.x() as f32);
            let offset_y =
                (start_row as f32).mul_add(tile_size as f32, bounds.y as f32 + -camera.y() as f32);

            for l in 0..map.layer_count {
                // add 1 to prevent missing tiles at the borders
                let mut end_column = end_column + 1;
                let mut end_row = end_row + 1;

                if end_column > map.column_count() {
                    end_column = map.column_count();
                }

                if end_row > map.row_count() {
                    end_row = map.row_count();
                }

                for r in start_row..end_row {
                    for c in start_column..end_column {
                        let tile = map.get_tile(l, r, c);

                        if tile == -1 {
                            continue;
                        }

                        let tile_column_count = image.width() as f32 / map.tile_size as f32;
                        let tile_c = tile as f32 % tile_column_count as f32;
                        let tile_r = (tile as f32 / tile_column_count as f32).floor();

                        let s_x = ((c - start_column) as f32)
                            .mul_add(map.tile_size as f32, offset_x as f32);
                        let s_y =
                            ((r - start_row) as f32).mul_add(map.tile_size as f32, offset_y as f32);

                        context.render_context_2_d().draw_image_with_clip(
                            image,
                            Rectangle::new(
                                bounds.x + tile_c as f64 * map.tile_size() as f64,
                                bounds.y + tile_r as f64 * map.tile_size() as f64,
                                map.tile_size as f64,
                                map.tile_size as f64,
                            ),
                            s_x as f64,
                            s_y as f64,
                        );
                    }
                }
            }
        }
    }
}
