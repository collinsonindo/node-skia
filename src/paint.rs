use skia_safe::Paint;

#[napi]
struct NSkiaPaint{
    paint:Paint
}


#[napi]
impl NSkiaPaint{
    #[napi(constructor)]
    pub fn new()->NSkiaPaint{
        NSkiaPaint{
            paint:Paint::default()
        }
    }    
    #[napi]
    pub fn set_anti_alias(&mut self,anti_alias:bool){
        self.paint.set_anti_alias(anti_alias);
    }
}