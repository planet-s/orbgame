use orbtk::prelude::*;

/// Used to build a camera, specifying additional details.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct CameraBuilder {
    rect: Rectangle,
    maximum: Point,
    speed: f64,
}

impl CameraBuilder {
    /// Creates a camera builder with default values.
    pub fn new() -> Self {
        CameraBuilder::default()
    }

    /// Inserts a x.
    pub fn x(mut self, x: f64) -> Self {
        self.rect.x = x;
        self
    }

    /// Inserts a y.
    pub fn y(mut self, y: f64) -> Self {
        self.rect.y = y;
        self
    }

    /// Inserts a width.
    pub fn width(mut self, width: f64) -> Self {
        self.rect.width = width;
        self
    }

    /// Inserts a height.
    pub fn height(mut self, height: f64) -> Self {
        self.rect.height = height;
        self
    }

    /// Inserts a max_width.
    pub fn max_width(mut self, max_width: f64) -> Self {
        self.maximum.x = max_width;
        self
    }

    /// Inserts a max_height.
    pub fn max_height(mut self, max_height: f64) -> Self {
        self.maximum.y = max_height;
        self
    }

    /// Inserts a speed.
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    /// Builds the camera.
    pub fn build(self) -> Camera {
        Camera {
            rect: self.rect,
            maximum: self.maximum,
            speed: self.speed,
        }
    }
}

/// The camera is use to describes the viewport on a screen like a part of a tile map.
///
/// The camera can be moved.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Camera {
    rect: Rectangle,
    maximum: Point,
    speed: f64,
}

into_property_source!(Camera);

impl Camera {
    /// Creates a camera builder with default values.
    pub fn create() -> CameraBuilder {
        CameraBuilder::new()
    }

    /// Creates a new camera.
    pub fn new(rect: Rectangle, maximum: Point) -> Self {
        Camera {
            rect,
            maximum,
            speed: 256.0,
        }
    }

    /// Gets x.
    pub fn x(&self) -> f64 {
        self.rect.x
    }

    /// Sets x.
    pub fn set_x(&mut self, x: f64) {
        self.rect.x = x;
    }

    /// Gets y.
    pub fn y(&self) -> f64 {
        self.rect.y
    }

    /// Sets y.
    pub fn set_y(&mut self, y: f64) {
        self.rect.y = y;
    }

    /// Gets position.
    pub fn position(&self) -> (f64, f64) {
        (self.rect.x, self.rect.y)
    }

    /// Sets position.
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.rect.x = x;
        self.rect.y = y;
    }

    /// Gets with.
    pub fn width(&self) -> f64 {
        self.rect.width
    }

    /// Sets width.
    pub fn set_width(&mut self, width: f64) {
        self.rect.width = width;
    }

    /// Gets height.
    pub fn height(&self) -> f64 {
        self.rect.height
    }

    /// Sets height.
    pub fn set_height(&mut self, height: f64) {
        self.rect.height = height;
    }

    /// Gets size.
    pub fn size(&self) -> (f64, f64) {
        (self.rect.width, self.rect.height)
    }

    /// Sets size.
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.rect.width = width;
        self.rect.height = height;
    }

    /// Gets maximum.
    pub fn maximum(&self) -> &Point {
        &self.maximum
    }

    /// Sets maximum.
    pub fn set_maximum(&mut self, x: f64, y: f64) {
        self.maximum.x = x;
        self.maximum.y = y;
    }

    /// Gets speed.
    pub fn speed(&self) -> f64 {
        self.speed
    }

    /// Sets speed.
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    /// Moves the camera.
    pub fn mov(&mut self, delta: f64, dir_x: f64, dir_y: f64) {
        self.rect.x += (dir_x as f64 * self.speed as f64 * delta) as f64;
        self.rect.y += (dir_y as f64 * self.speed as f64 * delta) as f64;

        let zero: f64 = 0.0;

        // adjust to respect the render_camera
        self.rect.x = zero.max(self.rect.x.min(self.maximum.x));
        self.rect.y = zero.max(self.rect.y.min(self.maximum.y));
    }

    // pub fn follow(&mut self, entity: &mut Entity) {
    //     let mut screen_position = entity.screen_position().get();
    //     let entity_rect = entity.rect().get();
    //     let mut rect = self.rect.get();
    //     let maximum = self.maximum.get();

    //     screen_position.x = rect.width as f64 / 2.0;
    //     screen_position.y = rect.height as f64 / 2;

    //     // make the camera follow the sprite
    //     rect.x = entity_rect.x - rect.width as f64 / 2;
    //     rect.y = entity_rect.y - rect.height as f64 / 2;

    //     let zero: f64 = 0;

    //     // clamp values
    //     rect.x = zero.max(rect.x.min(maximum.x));
    //     rect.y = zero.max(rect.y.min(maximum.y));

    //     // in map corners, the sprite cannot be placed in the center of the screen
    //     // and we have to change its screen coordinates

    //     // left and right sides
    //     if entity_rect.x < rect.width as f64 / 2
    //         || entity_rect.x > maximum.x + rect.width as f64 / 2
    //     {
    //         let new_x = entity_rect.x - rect.x;
    //         screen_position.x = new_x;
    //     }
    //     // top and bottom sides
    //     if entity_rect.y < rect.height as f64 / 2
    //         || entity_rect.y > maximum.y + rect.height as f64 / 2
    //     {
    //         let new_y = entity_rect.y - rect.y;
    //         screen_position.y = new_y;
    //     }

    //     entity.screen_position().set(screen_position);
    //     self.rect.set(rect);
    // }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_set_width() {
        let mut camera = Camera::default();
        camera.set_width(5.0);

        assert_eq!(5.0, camera.width());
    }

    #[test]
    fn test_set_height() {
        let mut camera = Camera::default();
        camera.set_height(5.0);

        assert_eq!(5.0, camera.height());
    }

    #[test]
    fn test_set_size() {
        let mut camera = Camera::default();
        camera.set_size(6.0, 7.0);

        assert_eq!(6.0, camera.width());
        assert_eq!(7.0, camera.height());
    }

    #[test]
    fn test_set_x() {
        let mut camera = Camera::default();
        camera.set_x(5.0);

        assert_eq!(5.0, camera.x());
    }

    #[test]
    fn test_set_y() {
        let mut camera = Camera::default();
        camera.set_y(5.0);

        assert_eq!(5.0, camera.y());
    }

    #[test]
    fn test_set_position() {
        let mut camera = Camera::default();
        camera.set_position(6.0, 7.0);

        assert_eq!(6.0, camera.x());
        assert_eq!(7.0, camera.y());
    }

    #[test]
    fn test_mov() {
        let mut camera = Camera::new(
            Rectangle::new(0.0, 0.0, 10.0, 10.0),
            Point::new(100.0, 50.0),
        );
        camera.mov(0.2, -10.0, -10.0);
        assert_eq!(0.0, camera.x());
        assert_eq!(0.0, camera.y());

        camera = Camera::new(
            Rectangle::new(0.0, 0.0, 10.0, 10.0),
            Point::new(100.0, 50.0),
        );
        camera.mov(1.0, 200.0, 200.0);
        assert_eq!(100.0, camera.x());
        assert_eq!(50.0, camera.y());
        camera = Camera::new(
            Rectangle::new(0.0, 0.0, 10.0, 10.0),
            Point::new(100.0, 50.0),
        );
        camera.mov(1.0, -10.0, 200.0);
        assert_eq!(0.0, camera.x());
        assert_eq!(50.0, camera.y());
    }

    #[test]
    fn test_builder() {
        assert_eq!(5.0, Camera::create().x(5.0).build().x());
        assert_eq!(6.0, Camera::create().y(6.0).build().y());
        assert_eq!(7.0, Camera::create().width(7.0).build().width());
        assert_eq!(8.0, Camera::create().height(8.0).build().height());
        assert_eq!(9.0, Camera::create().max_width(9.0).build().maximum().x);
        assert_eq!(10.0, Camera::create().max_height(10.0).build().maximum().y);
        assert_eq!(11.0, Camera::create().speed(11.0).build().speed());
    }
}
