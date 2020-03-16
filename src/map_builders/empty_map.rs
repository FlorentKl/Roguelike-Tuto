use super::{BuilderMap, InitialMapBuilder, TileType};
use rltk::RandomNumberGenerator;

pub struct EmptyMapBuilder {}

impl InitialMapBuilder for EmptyMapBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, rng: &mut rltk::RandomNumberGenerator, build_data: &mut BuilderMap) {
        self.build(rng, build_data);
    }
}

impl EmptyMapBuilder {
    #[allow(dead_code)]
    pub fn new() -> Box<EmptyMapBuilder> {
        Box::new(EmptyMapBuilder {})
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, _rng: &mut RandomNumberGenerator, build_data: &mut BuilderMap) {
        for y in 0..build_data.map.height - 1 {
            for x in 0..build_data.map.width - 1 {
                let idx = build_data.map.xy_idx(x, y);

                if y == 0
                    || y == build_data.map.height - 1
                    || x == 0
                    || x == build_data.map.width - 1
                {
                    build_data.map.tiles[idx] = TileType::Wall;
                }
                build_data.map.tiles[idx] = TileType::Floor;
            }
        }

        build_data.take_snapshot();
    }
}
