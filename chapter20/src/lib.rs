use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    queue: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(n_threads: usize) -> ThreadPool {
        assert!(n_threads > 0);

        let (tx, rx) = mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let workers = (0..n_threads)
            .into_iter()
            .map(|i| Worker::new(i, Arc::clone(&rx)))
            .collect();

        ThreadPool { workers, queue: tx }
    }

    pub fn exec<T>(&self, task: T)
    where
        T: FnOnce() + Send + 'static,
    {
        self.queue
            .send(Job {
                task: Box::new(task),
            })
            .unwrap()
    }
}

type Jobs = Arc<Mutex<mpsc::Receiver<Job>>>;
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, jobs: Jobs) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                let job = {
                    let lock = jobs.lock();
                    let rx = &*lock.unwrap();
                    rx.recv().unwrap()
                };
                println!("{} executing", id);
                job.exec();
            }),
        }
    }
}

struct Job {
    task: Box<FnOnce() + Send + 'static>,
}

impl Job {
    fn exec(self) {
        (self.task)();
    }
}
