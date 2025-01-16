// Game components
pub(crate) use bevy::prelude::*;
pub(crate) use bevy_rapier3d::prelude::*;

pub(crate) mod error {
    pub(crate) use crate::error::Error;

    pub type Result<T> = core::result::Result<T, Error>;

    // Generic Wrapper tuple struct for newtype pattern
    pub struct W<T>(pub T);
}

pub(crate) mod rocket {
    pub(crate) use crate::rocket::RocketControl;
}
