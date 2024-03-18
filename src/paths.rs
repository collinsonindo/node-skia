use napi::Error;
use skia_safe::Path;
use skia_safe::PathFillType;
#[napi]
pub enum NSkiaPathFillType {
  Winding = 0,
  EvenOdd = 1,
  InverseWinding = 2,
  InverseEvenOdd = 3,
}
impl NSkiaPathFillType {
  fn to(self) -> PathFillType {
    match self {
      Self::Winding => PathFillType::Winding,
      Self::EvenOdd => PathFillType::EvenOdd,
      Self::InverseEvenOdd => PathFillType::InverseEvenOdd,
      Self::InverseWinding => PathFillType::InverseWinding,
    }
  }
}
impl From<PathFillType> for NSkiaPathFillType {
  fn from(value: PathFillType) -> Self {
    match value {
      PathFillType::Winding => Self::Winding,
      PathFillType::EvenOdd => Self::EvenOdd,
      PathFillType::InverseEvenOdd => Self::InverseEvenOdd,
      PathFillType::InverseWinding => Self::InverseWinding,
    }
  }
}

#[napi]
pub struct NSkiaPath {
  pub (crate) path: Path,
}

#[napi]
impl NSkiaPath {
  /// Constructs an empty Path by default, path has no versbs, no Point and no weights
  #[napi(constructor)]
  pub fn new() -> NSkiaPath {
    NSkiaPath { path: Path::new() }
  }
  
  #[napi]
  pub fn is_interpolatable(&self, other: &NSkiaPath) -> bool {
    self.path.is_interpolatable(&other.path)
  }
  #[napi]
  pub fn fill_type(&self) -> NSkiaPathFillType {
    NSkiaPathFillType::from(self.path.fill_type())
  }
  #[napi]
  pub fn set_fill_type(&mut self, ft: NSkiaPathFillType) {
    self.path.set_fill_type(ft.to());
  }
  #[napi]
  pub fn is_convex(&self) -> bool {
    self.path.is_convex()
  }
  #[napi]
  pub fn is_finite(&self)->bool{
    self.path.is_finite()
  }
  #[napi]
  pub fn move_to(&mut self,x:f64,y:f64){
    // let x_conv:f32 =  x.try_into().map_err(|e| Error::new(napi::Status::InvalidArg,"Could not convert to f32") )?;
    // let y_conv:f32 = y.try_into().map_err(|e| Error::new(napi::Status::InvalidArg,"Could not convert to f32") )?;
    self.path.move_to((x as f32,y as f32));
  }
  #[napi]
  pub fn line_to(&mut self, x:f64,y:f64){
    self.path.line_to((x as f32,y as f32));
  }


  #[napi]
  pub fn quad_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) {
    self.path.quad_to((cpx as f32, cpy as f32), (x as f32, y as f32));
  }

  #[allow(dead_code)]
  #[napi]
  pub fn cubic_to(&mut self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
    self.path.cubic_to((cp1x as f32, cp1y as f32), (cp2x as f32, cp2y as f32), (x as f32, y as f32));
  }

  #[allow(dead_code)]
  #[napi]
  pub fn close_path(&mut self) {
    self.path.close();
  }

  pub fn close(&mut self){
    self.path.close();
  }
}
