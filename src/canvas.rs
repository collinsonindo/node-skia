use skia_safe::{Color4f, EncodedImageFormat, Surface, Point};


#[napi]
pub struct NSkiaSurfaces {
    surface: Surface,
}

#[napi]
impl NSkiaSurfaces {
    #[napi(constructor)]
    pub fn new_raster_n32_premul(width: i32, height: i32) -> NSkiaSurfaces {
        let surfaces = skia_safe::surfaces::raster_n32_premul((width, height)).expect("Cannot create surface");
        NSkiaSurfaces {
            surface: surfaces
        }
    }
    #[napi(factory)]
    pub fn new_null(width: i32, height: i32) -> NSkiaSurfaces {
        let surfaces = skia_safe::surfaces::null((width, height)).expect("Cannot create surface");
        NSkiaSurfaces {
            surface: surfaces
        }
    }

    #[napi]
    pub fn c_save(&mut self) -> i32 {
        self.surface.canvas().save() as i32
    }
    #[napi]
    pub fn c_clear(&mut self, color: u32) {
        self.surface.canvas().clear(Color4f::from_bytes_rgba(color));
    }


    #[napi]
    pub fn save_to(&mut self, file: String) {
        let image = self.surface.image_snapshot();
        let mut context = self.surface.direct_context();
        let encoded = image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .unwrap();
        std::fs::write(file, encoded.as_bytes()).expect("Could not save");
    }
    #[napi]
    pub fn im_width(&mut self) -> i32 {
        self.surface.image_info().width()
    }
    #[napi]
    pub fn im_height(&mut self) -> i32 {
        self.surface.image_info().width()
    }

    #[napi]
    pub fn c_translate(&mut self, x: i32, y: i32) {
        self.surface.canvas().translate(Point::from((x, y)));
    }
    #[napi]
    pub fn c_rotate(&mut self, degrees: f64, point_x: Option<i32>, point_y: Option<i32>) {
        if point_x.is_some_and(|_| point_y.is_some()) {
            self.surface.canvas().rotate(degrees as f32, Some(Point::from((point_x.unwrap(), point_y.unwrap()))));
        } else{
            self.surface.canvas().rotate(degrees as f32,None);
        }
    }
}