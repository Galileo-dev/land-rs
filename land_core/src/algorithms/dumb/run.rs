use std::collections::HashMap;
use std::hash::Hash;
use std::result;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use krpc_mars::batch_call_common;
use krpc_mars::batch_call_unwrap;
use krpc_mars::krpc::Stream;
use krpc_mars::krpc::StreamResult;
use krpc_mars::RPCClient;
use krpc_mars::StreamClient;
use krpc_mars::StreamHandle;
use krpc_mars::StreamUpdate;
use land_libs::*;
use log::info;

use crate::algo::State;

use super::Dumb;

struct Streams {
    vessel: Option<StreamHandle<land_libs::space_center::Vessel>>,
    ut_stream_handle: Option<StreamHandle<f64>>,
    velocity: Option<StreamHandle<(f64, f64, f64)>>,
    g: Option<StreamHandle<f32>>,
    surface_altitude: Option<StreamHandle<f64>>,
}

impl Streams {
    pub fn new(client: Arc<RPCClient>) -> Result<Streams, failure::Error> {
        let vessel = client.mk_call(&space_center::get_active_vessel()).unwrap();
        let surface_ref_frame = client
            .mk_call(&vessel.get_surface_reference_frame())
            .unwrap();
        let flight = client.mk_call(&vessel.flight(&surface_ref_frame)).unwrap();
        let orbit = client.mk_call(&vessel.get_orbit()).unwrap();
        let body = client.mk_call(&orbit.get_body()).unwrap();

        let (vessel, ut_stream_handle, velocity, g, surface_altitude) = batch_call_unwrap!(
            &client,
            (
                &space_center::get_active_vessel().to_stream(),
                &space_center::get_ut().to_stream(),
                &flight.get_velocity().to_stream(),
                &body.get_surface_gravity().to_stream(),
                &flight.get_surface_altitude().to_stream(),
            )
        )
        .unwrap();

        Ok(Streams {
            vessel: Some(vessel),
            ut_stream_handle: Some(ut_stream_handle),
            velocity: Some(velocity),
            g: Some(g),
            surface_altitude: Some(surface_altitude),
        })
    }
}

pub fn run(
    algo: &Dumb,
    mut state: Arc<Mutex<State>>,
    client: Arc<RPCClient>,
    stream_client: Arc<StreamClient>,
) -> Result<(), failure::Error> {
    let mut state = state.lock().unwrap();
    state.ETA;

    let streams = Streams::new(client.clone()).unwrap();

    loop {
        let update = stream_client.recv_update().unwrap();
        println!("{:?}", &streams.velocity.unwrap());
        let velocity = update.get_result(&streams.velocity.unwrap()).unwrap();
        println!("{:?}", velocity);
        // let active_vessel = client
        //     .clone()
        //     .mk_call(&space_center::get_active_vessel().to_stream())
        //     .unwrap();
        // let active_vessel = update.get_result(&active_vessel).unwrap();
        // println!("{:?}", active_vessel);
    }

    println!("Done");

    Ok(())
}

// fn get_value<T>(handle: &StreamHandle<T>, update: StreamUpdate) -> Result<Option<T>, Option<f32>> {
//     // Ok(update.get_result(&handle))
//     Ok(Option())
// }

// pub fn get_result<T>(&self, handle: &StreamHandle<T>) -> Result<T>
// where T: codec::RPCExtractable
// {
// let result = self.updates.get(&handle.stream_id).ok_or(Error::NoSuchStream)?;
// codec::extract_result(&result)
// }

// fn landing_calculations(velocity: (f32, f32, f32)) -> (f32) {}
