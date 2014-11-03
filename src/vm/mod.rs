pub use self::exec::VM;
pub use self::frame::Frame;

mod exec;
mod frame;
mod library;

pub type Stack = Vec<::gc::Value>;
