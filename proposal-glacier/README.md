# Summary
[summary]: #summary

A proposal for thread pool interface based on existing `future` API.

### Example: basic
Simple example to show the `Executor` trait in action. Uses two threadpools with seperated job queues.
```
cargo run --example basic
```

# Motivation
[motivation]: #motivation

We are aiming at trying to allow thread pool (or more general `executor`) implementers give maximum freedom to support different execution approaches as used in practice (e.g https://github.com/rust-gamedev/wg/issues/75#issuecomment-564972595)

On the other hand we need to keep the library authors in mind to provide an appealing API, in particular for user how not primarily care about gamedev at all.

Important aspects in gamedev:

- Full control over the thread pool creation and setup
- Support heterogeneous workloads (e.g IO tasks <-> high priority)

# Explanation
[explanation]: #explanation

Let's look at a simplified version of the `future` core API:
```rust
pub trait Executor {
    /// Schedule a new task for execution.
    fn spawn(&mut self, f: impl Task);
}

pub trait Task : Send + Sync + 'static {
    /// Execution of the task.
    fn poll(&mut self);
}
```

As library author, who 'designs' tasks, we would have to take an `Executor`.
The library consumer on the other hand needs to provide the corresponding executor when calling into the library.

#### "Full control over the thread pool creation and setup"

Given the above interface, we don't limit executor implementers in anyway regarding thread number, core pinning, priority or supporting more complex setups (e.g fibers, groups of threads, ..).

#### "Support heterogeneous workloads (e.g IO tasks <-> high priority)"

The example below should indicate how heterogeneous workloads can be handled.
This is reponsibility of the caller, the library author has no control over this!

```rust
fn important_work(executor: &mut impl Executor, desc: &str) {
    executor.spawn(Task { val: 10, desc: desc.into() });
}

fn heavy_work(executor: &mut impl Executor, desc: &str) {
    executor.spawn(Task { val: 1000, desc: desc.into() });
}

// ...

let mut executor = glacier::WorkQueue::new();

heavy_work(&mut executor.normal_queue());
important_work(&mut executor.high_queue());
```

# Drawbacks
[drawbacks]: #drawbacks

- On a similar note as allocators for examples, we need to explicitly pass the executors around, which bloats the function signature. Other languages like `dyon` (or `jai`?) have builtin support for context parameters providing syntactic sugar (see https://github.com/PistonDevelopers/dyon/issues/224).

- Futures may have additional overhead, which could be avoided with a more simplistic API.
