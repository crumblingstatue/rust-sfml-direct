#[allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::all
)]
mod raw {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod system;
