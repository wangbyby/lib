##多线程
  - move关键字
    ```rust
    use std::thread;
    fn main(){
        let v = vec![1,2,3,4];
        let handle = thread::spawn(move ||{
            for i in &v{
                println!("{}", i);
            }
        });
        for i in 12..=17{
            println!("{}", i);
        }
        handle.join().unwrap();
    }
    ```
   - channel
        ```rust
        use std::thread;
        use std::sync::mpsc;
        fn main(){
            let (sender, receiver) = mpsc::channel();
            let sender2 = sender.clone(); // 只有发送端有clone, 接收端没有:)
            let v = vec![1,2,3,4];
            let handle = thread::spawn(move ||{
                for i in v{
                    println!("{}", i);

                    //想象一个船从上游到下游
                    //消耗所有权
                    sender.send(i).unwrap(); 
                }
                
            });
            thread::spawn(move || {
                for i in 2..=5{
                    sender2.send(i).unwrap();
                }
                
            });
            for i in receiver {
                println!("{}", i);
            }
            
            handle.join().unwrap();
        }
        
        ```
  - 共享资源
    ```rust
    use std::thread;
    use std::sync::{Mutex, Arc,};
    fn main(){
        
        let counter = Arc::new(Mutex::new(0));
        
        let mut handlers = vec![];
        
        for _ in 0..10{
            let counter = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num +=1;
                
            });
            handlers.push(handler);
        }
        for i in handlers{
            i.join().unwrap();
        }
        println!("{}",*counter.lock().unwrap());
    }
    ```
  - 线程池
    ```rust
        use std::sync::mpsc;
        use std::sync::Arc;
        use std::sync::Mutex;
        use std::thread;

        const MIN_WORKERS: usize = 100;
        const MAX_WORKERS: usize = 1 << 16 - 1;

        macro_rules! set_workers {
            ($num:expr) => {
                match $num {
                    MIN_WORKERS ..= MAX_WORKERS => $num,
                    _=> MIN_WORKERS,
                }
            };
        }

        enum Message {
            NewJob(Job),
            Terminate,
        }

        pub struct ThreadPool {
            workers: Vec<Worker>,
            sender: mpsc::Sender<Message>,
        }

        type Job = Box<dyn FnOnce() + Send + 'static>;

        impl ThreadPool {
            pub fn new(cap: usize) -> ThreadPool {
                let size = set_workers!(cap);
                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));
                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver))); //接收者的副本...
                }
                ThreadPool { workers, sender }
            }
            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
                let job = Box::new(f);
                self.sender.send(Message::NewJob(job)).unwrap();
            }
        }
        impl Drop for ThreadPool {
            fn drop(&mut self) {
                for _ in &mut self.workers {
                    self.sender.send(Message::Terminate).unwrap(); //逐一发送停止消息
                }

                for worker in &mut self.workers {
                    println!("shutdown worker{}", worker.id);
                    if let Some(thread) = worker.thread.take() {
                        thread.join().unwrap(); //等待结束
                    }
                }
            }
        }

        struct Worker {
            id: usize,
            thread: Option<thread::JoinHandle<()>>,
        }

        impl Worker {
            fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
                let thread = thread::spawn(move || loop { //无限循环
                    let message = receiver.lock().unwrap().recv().unwrap();

                    match message {
                        Message::NewJob(job) => {
                            println!("Workers id {} got a job running", id);
                            job();
                        }
                        Message::Terminate => {
                            println!("Workers {}  was told  to terminate", id);
                            break;//结束信号
                        }
                    }
                });

                Worker {
                    id,
                    thread: Some(thread),
                }
            }
        }
    ```
