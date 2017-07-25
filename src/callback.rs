pub type CallbackNoArg  = Box<FnMut() + Send>;
pub type CallbackOneArg = Box<FnMut(String) + Send>;
pub type CallbackTwoArg = Box<FnMut(String, String) + Send>;

pub enum Callback {
    NoArg(CallbackNoArg),
    OneArg(CallbackOneArg),
    TwoArg(CallbackTwoArg),
}