use crate::{api::prelude::TileMapRenderObject, prelude::*, utils::*};

widget!(
    /// The `TileMap` widget is use to draw a tile map to the screen an to navigate on the map with a camera.
    TileMap {
        /// Sets or shares the map property.
        map: Map,

        /// Sets or shares the camera of the tile map.
        camera: Camera,

        /// Sets or shares the image of the tile map.
        image: Image
    }
);

impl Template for TileMap {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("TileMap")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(TileMapRenderObject)
    }
}
