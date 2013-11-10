use gc::Env;

pub struct Frame {
    env: Env,
    sp: uint,
    pc: uint,
    caller: Option<~Frame> 
}

