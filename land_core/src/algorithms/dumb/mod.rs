use krpc_mars::krpc::Stream;
use krpc_mars::RPCClient;
use krpc_mars::StreamClient;

use crate::algo::Script;
use crate::algo::State;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

mod exit;
mod run;

pub struct Dumb {
    // streams: Stream,
}

impl Script for Dumb {
    fn run(
        &self,
        mut state: Arc<Mutex<State>>,
        client: Arc<RPCClient>,
        stream_client: Arc<StreamClient>,
    ) -> Result<(), failure::Error> {
        run::run(self, state, client, stream_client);

        Ok(())
    }

    fn exit(&self) -> Result<(), failure::Error> {
        exit::exit();
        Ok(())
    }
}

pub fn dumb_algorithm() -> Dumb {
    Dumb {}
}
