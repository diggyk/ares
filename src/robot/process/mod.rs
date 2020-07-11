pub enum ProcessResult {
    Ok,
    Fail,
    TransitionProcess(String),
}

pub trait Process {
    fn run(&self);
}