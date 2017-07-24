pub type CallbackNoArg  = Box<FnMut()>;
pub type CallbackOneArg = Box<FnMut(String)>;
pub type CallbackTwoArg = Box<FnMut(String, String)>;

pub enum Callback {
    NoArg(CallbackNoArg),
    OneArg(CallbackOneArg),
    TwoArg(CallbackTwoArg),
}