//use core::algo::STATUS;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

extern crate land_libs;
use core::algo::Algorithm;
use core::algo::ComputeUnit;
use core::algorithms::dumb::dumb_algorithm;
use core::algorithms::dumb::Dumb;
use std::sync::Arc;

use krpc_mars::batch_call_common;
use krpc_mars::batch_call_unwrap;
use land_libs::*;

extern crate krpc_mars;

extern crate failure;

fn main() -> Result<(), failure::Error> {
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
    info!("Initializing Client...");
    let client = Arc::new(krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?);
    let stream_client = Arc::new(krpc_mars::StreamClient::connect(
        &client,
        "127.0.0.1:50001",
    )?);
    info!("Client Initialization Complete!");

    let mut compute_unit = ComputeUnit::new(client, stream_client);

    let dumb_script = Arc::new(dumb_algorithm());
    let dumb_algorithm = Algorithm::start(dumb_script).build();

    compute_unit.add_algorithm(dumb_algorithm);
    compute_unit.start_algorithm();

    // println!("Current vessel: {:?}", vessel);

    // loop {
    //     let update = stream_client.recv_update()?;
    //     let ut_result = update.get_result(&ut_stream_handle)?;
    //     println!("ut: {}", ut_result);
    // }
    Ok(())
}
