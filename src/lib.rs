use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// ThreadPoolは、スレッドを直接保持するのではなく、Workerインスタンスを保持するようにした
// また、Jobインスタンスを送信するチャンネルの送信側を格納した
pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

// Jobはexecuteが受け取るクロージャの型を保持するトレイトオブジェクトの型エイリアス
// 型エイリアスにより長い型を短くできる
// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// チャンネルの受信側をワーカーに渡す
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
                                                            loop {
                                                                // まずreceiverに対してlockを呼び出してミューテックスを獲得
                                                                // recvを呼び出してチャンネルからJobを受け取る
                                                                let job = receiver.lock().unwrap().recv().unwrap();

                                                                // ワーカー{id} は仕事を得ました。実行します。
                                                                println!("Worker {} got a job; executing.", id);
                                                                // (*job)();
                                                                job.call_box();
                                                            }
                                                        }
                                                );

        Worker { id, 
                thread, 
                }
    }
}


impl ThreadPool {
    /// 新しいThreadPoolを生成する
    /// sizeがプールのスレッド数です。
    /// sizeが0なら、`new`関数はパニックします。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // with_capacityは、ベクタに予めスペースを確保する。
        // (ベクタにsize個の要素を格納する必要があることはわかっているので)
        // Vec::newよりも少しだけ効率的。
        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // スレッドを生成してベクタに格納する
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            // threads
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
        where
            // クロージャーを引数として受け取る場合は、3種類のトレイトがある: Fn、FnMut、FnOnce
            // 今回はFnOnce:                リクエストのクロージャを1回だけ実行したいからOnce。
            // FnOnceの後に()を使用しているのは、引数を取らず、値も返さないクロージャを表すから。
            // トレイト境界のSend:          あるスレッドから別のスレッドにクロージャを移動する
            // ライフタイム境界の'static:   スレッドの実行にどれくらいかかるかわからないため
            F: FnOnce() + Send + 'static,
            // T: Send + 'static
    {
        let job = Box::new(f);
        // unwrapを使用している理由は、失敗する場合が起こらないとわかっているから。
        self.sender.send(job).unwrap();
    }
}

// 参考: executeメソッドは、thread::spawn実装にしたいと思っている。
// なのでspawnメソッドを参考にして、引数や戻り値の型を決める！
// この場合は、FnOnceとSendとJoinHandle
// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T + Send + 'static,
//         T: Send + 'static