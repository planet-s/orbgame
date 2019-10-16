use crate::{
    prelude::*,
    utils::{MapExt, Point, Position, Size},
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
                widget.get::<Bounds>().0.clone(),
                widget.get::<Camera>().0.clone(),
                widget.get::<Map>().0.clone(),
                widget.try_clone::<Image>(),
            )
        };

        if let Some(image) = &mut image {
            // draw the tile map
            let mut start_column = 0;
            let mut end_column = 0;
            let mut start_row = 0;
            let mut end_row = 0;
            let mut offset_x = 0.;
            let mut offset_y = 0.;

            let tile_size = map.tile_size;

            start_column = (camera.x() as f32 / tile_size as f32).floor() as usize;
            end_column = start_column + (camera.width() as f32 / tile_size as f32).ceil() as usize;
            start_row = (camera.y() as f32 / tile_size as f32).floor() as usize;
            end_row = start_row + (camera.height() as f32 / tile_size as f32).ceil() as usize;
            offset_x =
                bounds.x as f32 + -camera.x() as f32 + start_column as f32 * tile_size as f32;
            offset_y = bounds.y as f32 + -camera.y() as f32 + start_row as f32 * tile_size as f32;

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

                        let s_x = (((c - start_column) as f32) * map.tile_size as f32
                            + offset_x as f32) as i32;
                        let s_y = (((r - start_row) as f32) * map.tile_size as f32
                            + offset_y as f32) as i32;
                        let s_width = tile_c as u32 * map.tile_size;
                        let s_height = tile_r as u32 * map.tile_size;

                        context.render_context_2_d().draw_image_with_clip(
                            &mut image.0,
                            tile_c as f64 * map.tile_size() as f64,
                            tile_r as f64 * map.tile_size() as f64,
                            map.tile_size as f64,
                            map.tile_size as f64,
                            s_x as f64,
                            s_y as f64,
                        );
                    }
                }
            }
        }
    }
}
