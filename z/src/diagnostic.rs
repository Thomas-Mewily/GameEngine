use super::*;

pub use macroquad::prelude::{debug,info,warn,error};

#[macro_export(local_inner_macros)]
macro_rules! infoln 
{
    () => (
        info!("\n");
    );
    (target: $target:expr, $($arg:tt)+) => (
        info!(target: $target, "{}\n", format!($($arg)+));
    );
    ($($arg:tt)+) => (
        info!("{}\n", std::format!($($arg)+));
    )
}

#[macro_export(local_inner_macros)]
macro_rules! debugln 
{
    () => (
        debug!("\n");
    );
    (target: $target:expr, $($arg:tt)+) => (
        debug!(target: $target, "{}\n", format!($($arg)+));
    );
    ($($arg:tt)+) => (
        debug!("{}\n", std::format!($($arg)+));
    )
}

#[macro_export(local_inner_macros)]
macro_rules! warnln 
{
    () => (
        warn!("\n");
    );
    (target: $target:expr, $($arg:tt)+) => (
        warn!(target: $target, "{}\n", format!($($arg)+));
    );
    ($($arg:tt)+) => (
        warn!("{}\n", std::format!($($arg)+));
    )
}

#[macro_export(local_inner_macros)]
macro_rules! errorln 
{
    () => (
        error!("\n");
    );
    (target: $target:expr, $($arg:tt)+) => (
        error!(target: $target, "{}\n", format!($($arg)+));
    );
    ($($arg:tt)+) => (
        error!("{}\n", std::format!($($arg)+));
    )
}