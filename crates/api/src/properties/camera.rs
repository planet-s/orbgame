use crate::{
    prelude::*,
    utils::{Camera as CameraValue, CameraExt, Point, Position, Size},
};

property!(
    ///  The camera property is use to describes the viewport on a screen like a part of a tile map.
    Camera(CameraValue)
);

impl CameraExt for Camera {
    fn maximum(&self) -> &Point {
        self.0.maximum()
    }

    fn set_maximum(&mut self, x: f64, y: f64) {
        self.0.set_maximum(x, y);
    }

    fn speed(&self) -> f64 {
        self.0.speed()
    }

    fn set_speed(&mut self, speed: f64) {
        self.0.set_speed(speed);
    }

    fn mov(&mut self, delta: f64, dir_x: f64, dir_y: f64) {
        self.0.mov(delta, dir_x, dir_y);
    }
}

impl Size for Camera {
    fn width(&self) -> f64 {
        self.0.width()
    }

    fn set_width(&mut self, width: f64) {
        self.0.set_width(width);
    }

    fn height(&self) -> f64 {
        self.0.height()
    }

    fn set_height(&mut self, height: f64) {
        self.0.set_height(height)
    }

    fn size(&self) -> (f64, f64) {
        self.0.size()
    }

    fn set_size(&mut self, width: f64, height: f64) {
        self.0.set_size(width, height);
    }
}

impl Position for Camera {
    fn x(&self) -> f64 {
        self.0.x()
    }

    fn set_x(&mut self, x: f64) {
        self.0.set_x(x);
    }

    fn y(&self) -> f64 {
        self.0.y()
    }

    fn set_y(&mut self, y: f64) {
        self.0.set_y(y);
    }

    fn position(&self) -> (f64, f64) {
        self.0.position()
    }

    fn set_position(&mut self, x: f64, y: f64) {
        self.0.set_position(x, y);
    }
}
