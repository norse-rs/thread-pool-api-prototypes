
mod thread_pool;
pub use thread_pool::*;

/// Task executor API comparable to https://doc.rust-lang.org/1.29.2/std/task/trait.Executor.html.WorkQueue
pub trait Executor {
    fn spawn(&mut self, f: impl Task);
}

/// Task trait comparable to `Future`.
pub trait Task : Send + Sync + 'static {
    fn poll(&mut self);
}
