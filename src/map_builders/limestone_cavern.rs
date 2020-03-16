use super::{
    AreaStartingPosition, BuilderChain, BuilderMap, CullUnreachable, DistantExit,
    DrunkardsWalkBuilder, MetaMapBuilder, TileType, VoronoiSpawning, XStart, YStart,
};
use crate::map;
use rltk::RandomNumberGenerator;

pub fn limestone_cavern_builder(
    new_depth: i32,
    _rng: &mut rltk::RandomNumberGenerator,
    width: i32,
    height: i32,
) -> BuilderChain {
    let mut chain = BuilderChain::new(new_depth, width, height, "Limestone Caverns");
    chain.start_with(DrunkardsWalkBuilder::winding_passages());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::LEFT, YStart::CENTER));
    chain.with(VoronoiSpawning::new());
    chain.with(DistantExit::new());
    chain
}
