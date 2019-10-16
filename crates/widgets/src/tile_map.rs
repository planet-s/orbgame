use crate::prelude::*;

widget!(
    /// The `TileMap` widget is use to draw a tile map to the screen an to navigate on the map with a camera.
    TileMap {
        /// Sets or shared the camera of the tile map.
        camera: Camera,

        /// Sets or shared the image of the tile map.
        image: Image
    }
);

impl Template for TileMap {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TileMap")
    }
}
