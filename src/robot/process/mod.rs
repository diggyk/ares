use diesel::PgConnection;

mod neutral;
mod scan;

pub use neutral::Neutral;
pub use scan::Scan;


/// Message returned from a process run to let robot know what to do next
#[derive(Debug)]
pub enum ProcessResult {
    /// Ok means the process succeeded and should remain the active process
    Ok,
    /// Indicates the process failed
    Fail,
    /// Indicates that the process should be changed on the next run
    TransitionProcess(Processes),
}

/// List of all the processes with helpers to run the process
#[derive(Debug)]
pub enum Processes {
    Neutral,
    Scan,
}

impl Processes {
    pub fn run(&self, conn: &PgConnection) -> ProcessResult {
        match self {
            Self::Neutral => Neutral::run(conn),
            Self::Scan => Scan::run(conn),
        }
    }
}

/// Trait to define a process
pub trait Process {
    fn run(conn: &PgConnection) -> ProcessResult;
}