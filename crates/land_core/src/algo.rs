use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
    time::Duration,
};

use krpc_mars::{CallHandle, RPCClient, StreamClient};
use log::info;

pub struct Algorithm {
    status: STATUS,
    script: Arc<dyn Script>,
    isSpawned: bool,
    name: String,
    description: String,
}

impl Algorithm {
    pub fn start(script: Arc<dyn Script>) -> AlgorithmBuilder {
        info!("Starting New Algorithm...");
        AlgorithmBuilder {
            name: String::new(),
            description: String::new(),
            script,
        }
    }
}

pub struct AlgorithmBuilder {
    name: String,
    description: String,
    script: Arc<dyn Script>,
}

impl AlgorithmBuilder {
    pub fn build(self) -> Algorithm {
        Algorithm {
            script: self.script,
            status: STATUS::UNKNOWN,
            isSpawned: false,
            name: self.name,
            description: self.description,
        }
    }
}

pub struct ComputeUnit {
    client: Arc<RPCClient>,
    stream_client: Arc<StreamClient>,
    active_algorithm: Vec<Arc<Mutex<ThreadAlgorithmCombo>>>,
}

pub struct State {
    pub Status: Option<STATUS>,
    pub client_name: Option<String>,
    pub current_throttle: Option<f32>,
    pub current_altitude: Option<f32>,
    pub ETA: Option<f32>,
}

impl ComputeUnit {
    pub fn new(client: Arc<RPCClient>, stream_client: Arc<StreamClient>) -> ComputeUnit {
        ComputeUnit {
            client,
            stream_client,
            active_algorithm: vec![],
        }
    }

    // pub fn add_client(&mut self, client: RPCClient) {
    //     self.active_algorithm.push(combo.clone());
    // }

    // pub fn add_stream_client(&mut self, stream_client: RPCClient) {
    //     self.active_algorithm.push(combo.clone());
    // }

    pub fn add_algorithm(&mut self, algorithm: Algorithm) {
        let combo = Arc::new(Mutex::new(new_combo(algorithm, None)));
        self.active_algorithm.push(combo.clone());
    }

    pub fn start_algorithm(&mut self) {
        self.start();
    }

    fn start(&mut self) {
        let mut threads = vec![];
        let state = Arc::new(Mutex::new(State {
            Status: None,
            client_name: None,
            current_altitude: None,
            current_throttle: None,
            ETA: None,
        }));
        for i in 0..self.active_algorithm.len() {
            let threadcombo = self.active_algorithm[i].clone();

            //let script = self.active_algorithm[i].algorithm.script.clone();

            let client = self.client.clone();
            let stream_client = self.stream_client.clone();
            let state = Arc::clone(&state);
            let new_thread = thread::spawn(move || {
                threadcombo.lock().unwrap().algorithm.script.run(
                    state,
                    client.clone(),
                    stream_client.clone(),
                )
            });

            threads.push(new_thread);
        }

        for thread in threads {
            println!("{:?}", thread);
            thread.join();
            println!("Threads Joined");
        }
        // println!("{}", *state.lock().unwrap());
    }
}

pub struct ThreadAlgorithmCombo {
    thread: Option<JoinHandle<Result<(), failure::Error>>>,
    algorithm: Algorithm,
}

pub fn new_combo(
    algorithm: Algorithm,
    thread: Option<JoinHandle<Result<(), failure::Error>>>,
) -> ThreadAlgorithmCombo {
    ThreadAlgorithmCombo { algorithm, thread }
}

pub trait Script: Sync + Send {
    fn run(
        &self,
        state: Arc<Mutex<State>>,
        client: Arc<RPCClient>,
        StreamClient: Arc<StreamClient>,
    ) -> Result<(), failure::Error>;
    fn exit(&self) -> Result<(), failure::Error>;
}

pub enum STATUS {
    READY,
    RUNNING,
    STOPPED,
    ERROR,
    UNKNOWN,
}
