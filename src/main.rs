use amethyst::{
    core::transform::TransformBundle,
    ecs::prelude::{ReadExpect, Resource, SystemData},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct Pong;

impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    Ok(())
}
