use derivative::Derivative;
use serde::Serialize;


#[derive(Derivative, Serialize,Default)]
pub struct Result <T>{
    #[derivative(Default(value = "0"))]
    pub code: u32,

    pub msg:String,

    pub data:Option<T>,
}
