use gfold_rs::trajectories::{APDGSettings, APDGTrajectory};

fn main() -> Result<(), gfold_rs::trajectories::APDGError> {
    let settings = APDGSettings::builder()
        .landing_site([3.0, 4.0])
        .max_control(5.0)
        .build();

    let mut apdg = APDGTrajectory::new(settings)?;
    apdg.solve();
    Ok(())
}
