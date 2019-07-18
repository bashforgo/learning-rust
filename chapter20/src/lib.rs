use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
    queue: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(n_threads: usize) -> ThreadPool {
        assert!(n_threads > 0);

        let (tx, rx) = mpsc::channel();
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
        self.queue.send(Message::Job(Job(Box::new(task)))).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.queue.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

enum Message {
    Job(Job),
    Terminate,
}
type Jobs = Arc<Mutex<mpsc::Receiver<Message>>>;
struct Worker {
    _id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(_id: usize, jobs: Jobs) -> Worker {
        Worker {
            _id,
            thread: Some(thread::spawn(move || loop {
                let message = {
                    let lock = jobs.lock();
                    let rx = &*lock.unwrap();
                    rx.recv().unwrap()
                };

                match message {
                    Message::Job(job) => job.exec(),
                    Message::Terminate => break,
                }
            })),
        }
    }
}

struct Job(Box<FnOnce() + Send + 'static>);

impl Job {
    fn exec(self) {
        (self.0)();
    }
}
