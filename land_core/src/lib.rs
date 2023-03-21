pub mod algo;
pub mod algorithms;

// extern crate krpc_mars;
// #[allow(unused)]
// extern crate land_libs;
// use land_libs::*;
// extern crate failure;

// fn main() -> Result<(), failure::Error> {
//     let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000")?;
//     let stream_client = krpc_mars::StreamClient::connect(&client, "127.0.0.1:50001")?;

//     // let (vessel, ut_stream_handle) = batch_call_unwrap!(&client, (
//     //     &space_center::get_active_vessel(),
//     //     &space_center::get_ut().to_stream(),
//     // ))?;

//     // // let get_altitude = client.mk_call(&flight_telem.get_surface_altitude().to_stream())?;
//     // let auto_pilot = client.mk_call(&vessel.get_auto_pilot())?;
//     let vessel = client.mk_call(&space_center::get_active_vessel())?;
//     let orbit = client.mk_call(&vessel.get_orbit())?;
//     let control = client.mk_call(&vessel.get_control())?;
//     let body = client.mk_call(&orbit.get_body())?;
//     let reference_frame = client.mk_call(&body.get_orbital_reference_frame())?;
//     let flight_telem = client.mk_call(&vessel.flight(&reference_frame))?;
//     let auto_pilot = client.mk_call(&vessel.get_auto_pilot())?;
//     let parts = client.mk_call(&vessel.get_parts())?;
//     // let ( g_handle) = batch_call_unwrap!(&client, (

//     //     &body.get_surface_gravity().to_stream(),
//     // ))?;
//     let vel = client.mk_call(&flight_telem.get_vertical_speed().to_stream())?;

//     //=================================================== Handlers ===================================================

//     // Surface gravity

//     //TODO(Galileo-dev): Make g into stream (Currently returns no such stream)
//     let g = client.mk_call(&body.get_surface_gravity())? as f64;

//     //Vertical Speed
//     let vertical_speed_handler = client.mk_call(&flight_telem.get_vertical_speed().to_stream())?;

//     // Height above terrain

//     //TODO(): Causing errors NO SUCH STREAM
//     // let radar_handler = client.mk_call(&flight_telem.get_mean_altitude().to_stream())?;

//     // Available thrust
//     //TODO(): Causing errors NO SUCH STREAM
//     // let available_thrust_handler =  client.mk_call(&vessel.get_available_rcs_torque().to_stream())?;

//     //TODO(): Causing errors NO SUCH STREAM
//     // Ship altitude
//     // let ship_altitude_handler = client.mk_call(&flight_telem.get_mean_altitude().to_stream())?;

//     //Ship mass
//     //TODO(): Causing errors NO SUCH STREAM
//     // let ship_mass_handler = client.mk_call(&vessel.get_mass().to_stream())?;

//     //--------------------------------------------------------------------------------------------------------------------
//     //====================================================================================================================

//     let mut are_engines_firing = false;
//     // let height_above_ground = 50.0;
//     client.mk_call(&control.set_rcs(true))?;

//     loop {
//         let update = stream_client.recv_update()?;
//         let ut_result = client.mk_call(&space_center::get_ut())?;

//         // let vertical_speed = update.get_result(&vertical_speed_handler)?;

//         let vertical_speed = client.mk_call(&flight_telem.get_vertical_speed())?;
//         //=================================================== Get vars ===================================================
//         //=========== we will be converting all valuese to kila newtons (Formula divide the force value by 1000) =========
//         //Body radius
//         let body_radius = client.mk_call(&orbit.get_radius())?;
//         //Vertical Speed

//         // Height above terrain
//         let radar = client.mk_call(&flight_telem.get_mean_altitude())?;

//         // Available thrust

//         //?Problem where available_thrust = 0 when toggling engine mode
//         let ag1 = client.mk_call(&control.get_action_group(1))?;
//         let ag2 = client.mk_call(&control.get_action_group(2))?;

//         let mut available_thrust: f64 = 0.0;
//         if ag1 || ag2 {
//             //? Known value of thrust for different engine mode of octaweb
//             let full_thrust = 2368.1485;
//             available_thrust = if ag1 && ag2 {
//                 full_thrust / 9.0
//             } else {
//                 (full_thrust / 9.0) * 3.0
//             };
//         } else {
//             available_thrust = client.mk_call(&vessel.get_available_thrust())? as f64 / 1000.0;
//         }

//         //Ship mass
//         let ship_mass = client.mk_call(&vessel.get_mass())? as f64 / 1000.0;

//         //Calculate distance above ground
//         // TODO: ship alt is measured from center of mass of the vessel.
//         // Ship altitude
//         let vessel_height_center_of_mass = 6.432935519493185;
//         let ship_altitude =
//             client.mk_call(&flight_telem.get_surface_altitude())? - vessel_height_center_of_mass;

//         //--------------------------------------------------------------------------------------------------------------------
//         //====================================================================================================================

//         //=================================================== Calculations ===================================================
//         let max_decel = (available_thrust / ship_mass) - g; // Maximum deceleration possible (m/s^2)
//         let stop_dist = vertical_speed.powi(2) / (2.0 * max_decel); // The distance the burn will require
//         let ideal_Throttle = stop_dist / ship_altitude; // Throttle required for perfect hoverslam
//         let impact_Time = ship_altitude / vertical_speed.abs(); // Time until impact, used for landing gear

//         //====================================================================================================================

//         if stop_dist * 2.0 > ship_altitude && ideal_Throttle > 0.8 {
//             are_engines_firing = true;
//         }

//         if are_engines_firing {
//             client.mk_call(&control.set_throttle(ideal_Throttle as f32))?;
//         }

//         if vertical_speed > 0.0 {
//             break;
//         }

//         println!(
//             "max_decel:{}, stop_dist:{}, available_thrust:{}",
//             max_decel, stop_dist, available_thrust
//         );
//         println!(
//             "ut: {}, ideal_throttle: {}, radar: {}",
//             ut_result, ideal_Throttle, ship_altitude
//         );
//     }

//     client.mk_call(&control.set_throttle(0.0))?;
//     are_engines_firing = false;

//     Ok(())
// }

// // until false {

// //     lock trueRadar to bounds_box:BOTTOMALTRADAR.

// //         lock g to constant:g * body:mass / body:radius^2.		// Gravity (m/s^2)
// //         lock maxDecel to (ship:availablethrust / ship:mass) - g.	// Maximum deceleration possible (m/s^2)
// //         lock stopDist to (ship:verticalspeed^2 / (2 * maxDecel)).		// The distance the burn will require

// //     print("time" + TIME + "stop dist" + stopDist).
// //     wait 0.001.
// //     }
