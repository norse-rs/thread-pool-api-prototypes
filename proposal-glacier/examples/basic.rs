use proposal_glacier as glacier;
use glacier::Executor;

pub struct Task {
    val: usize,
    desc: String,
}

impl glacier::Task for Task {
    fn poll(&mut self) {
        println!("{}: {:?}", &self.desc, self.val);
        std::thread::sleep_ms(self.val as _);
    }
}

fn important_work(executor: &mut impl Executor, desc: &str) {
    executor.spawn(Task { val: 10, desc: desc.into() });
}

fn heavy_work(executor: &mut impl Executor, desc: &str) {
    executor.spawn(Task { val: 1000, desc: desc.into() });
}

fn main() {
    let mut normal_queue = glacier::WorkQueue::new(8);
    let mut prio_queue = glacier::WorkQueue::new(2);

    for i in 0..16 {
        heavy_work(&mut normal_queue, "heavy..".into());
    }

    for i in 0..4 {
        important_work(&mut prio_queue, "!");
    }

    loop { } // do all tasks
}
