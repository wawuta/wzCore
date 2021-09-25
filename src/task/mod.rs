use crate::io::port::Port;
use crate::ipc::channel::Channel;

use super::*;

pub mod exception;
pub mod job;
pub mod job_policy;
pub mod process;
pub mod thread;

/// Task (Thread, Process, or Job)
pub trait Task {
    /// Kill the task.
    fn kill(&mut self) -> ZxResult<()>;

    /// Supspend the task. Currently only thread or process handles may be suspended.
    fn suspend(&mut self) -> ZxResult<()>;

    /// Create an exception channel on the task.
    fn create_exception_channel(&mut self, options: u32) -> ZxResult<Channel>;

    /// Resume the task from a previously caught exception.
    fn resume_from_exception(&mut self, port: &Port, options: u32) -> ZxResult<()>;
}
