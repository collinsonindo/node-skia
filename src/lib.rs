#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;


mod paths;
mod canvas;
mod paint;

use skia_safe::{surfaces, Color, EncodedImageFormat, Paint, PaintStyle, Path, Surface};
use std::mem;
use crate::paths::NSkiaPath;

#[napi]
pub struct SkiaCanvas {
  surface: Surface,
  path: NSkiaPath,
  paint: Paint,
}

#[napi]
impl SkiaCanvas {
  #[napi(constructor)]
  pub fn new(width: i32, height: i32) -> SkiaCanvas {
    let mut surface = surfaces::raster_n32_premul((width, height)).expect("surface");
    let path = NSkiaPath::new();
    let mut paint = Paint::default();
    paint.set_color(Color::BLACK);
    paint.set_anti_alias(true);
    paint.set_stroke_width(1.0);
    surface.canvas().clear(Color::WHITE);
    SkiaCanvas {
      surface,
      path,
      paint,
    }
  }
  #[napi]
  pub fn save(&mut self) {
    self.canvas().save();
  }

  #[napi]
  pub fn translate(&mut self, dx: f64, dy: f64) {
    self.canvas().translate((dx as f32, dy as f32));
  }

  #[napi]
  pub fn scale(&mut self, sx: f64, sy: f64) {
    self.canvas().scale((sx as f32, sy as f32));
  }

  #[napi]
  pub fn move_to(&mut self, x: f64, y: f64) {
    self.begin_path();
    self.path.move_to(x, y);
  }

  #[napi]
  pub fn line_to(&mut self, x: f64, y: f64) {
    self.path.line_to(x, y );
  }

  #[napi]
  pub fn quad_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
    self.path.quad_to(cpx,cpy,x,y);
  }

  #[allow(dead_code)]
  #[napi]
  pub fn bezier_curve_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
    self.path.cubic_to(cp1x,cp1y,cp2x,cp2y,x,y);
  }

  #[allow(dead_code)]
  #[napi]
  pub fn close_path(&mut self) {
    self.path.close();
  }

  #[napi]
  pub fn begin_path(&mut self) {
    let new_path = NSkiaPath::new();
    self.surface.canvas().draw_path(&self.path.path, &self.paint);
    let _ = mem::replace(&mut self.path, new_path);
  }

  #[napi]
  pub fn stroke(&mut self) {
    self.paint.set_style(PaintStyle::Stroke);
    self.surface.canvas().draw_path(&self.path.path, &self.paint);
  }

  #[napi]
  pub fn fill(&mut self) {
    self.paint.set_style(PaintStyle::Fill);
    self.surface.canvas().draw_path(&self.path.path, &self.paint);
  }

  #[napi]
  pub fn set_line_width(&mut self, width: f64) {
    self.paint.set_stroke_width(width as f32);
  }
  
  #[napi]
  pub fn save_to(&mut self, file:String){
    
    let image = self.surface.image_snapshot();
    let mut context = self.surface.direct_context();
    let encoded = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, None)
        .unwrap();
    std::fs::write(file, encoded.as_bytes()).expect("Could not save");
  }
  #[inline]
  fn canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }
  
}

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
