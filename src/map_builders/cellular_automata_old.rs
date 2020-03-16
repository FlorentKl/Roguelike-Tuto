use super::{MapBuilder, Map, 
    TileType, Position, spawner, SHOW_MAPGEN_VISUALIZER, common::*};
use rltk::RandomNumberGenerator;
use specs::prelude::*;
use std::collections::HashMap;

pub struct CellularAutomataBuilder {
    map: Map,
    starting_position: Position,
    depth: i32,
    history: Vec<Map>,
    flood_filled: Vec<usize>,
    noise_areas: HashMap<i32, Vec<usize>>,
    spawn_list: Vec<(usize, String)>,
}

impl MapBuilder for CellularAutomataBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self)  {
        self.build();
    }

    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            for v in snapshot.revealed_tiles.iter_mut() {
                *v = true;
            }
            self.history.push(snapshot);
        }
    }

    fn get_spawn_list(&self) -> &Vec<(usize, String)> {
        &self.spawn_list
    }
}

impl CellularAutomataBuilder {
    #[allow(dead_code)]
    pub fn new(new_depth : i32) -> CellularAutomataBuilder {
        CellularAutomataBuilder{
            map: Map::new(new_depth),
            starting_position: Position{ x: 0, y : 0 },
            depth: new_depth,
            history: Vec::new(),
            flood_filled: Vec::new(),
            noise_areas: HashMap::new(),
            spawn_list: Vec::new(),
        }
    }

    fn build(&mut self) {
        //let newmap = self.map.clone();
        let mut gen_new_map = true;
        
        while gen_new_map{
            self.map = Map::new(self.depth);
            self.starting_position = Position{ x: 0, y: 0 };
            self.flood_filled = Vec::new();
            self.history = Vec::new();

            let mut rng = RandomNumberGenerator::new();
            //Randomise map, setting 45% of tiles as Wall
            for y in 1..self.map.height - 1{
                for x in 1..self.map.width - 1{
                    let roll = rng.roll_dice(1, 100);
                    let idx = self.map.xy_idx(x, y, 9);
                    if roll <= 45 { self.map.tiles[idx] = TileType::Wall }
                    else { self.map.tiles[idx] = TileType::Floor }
                }
            }
            self.take_snapshot();
            
            //Cellular automata rules
            for _i in 0..12 { //16 itération de l'algorithme
                let mut newtiles = self.map.tiles.clone();

                if _i <= 9 {
                    for y in 1..self.map.height - 1 {
                        for x in 1..self.map.width - 1 {
                            let idx = self.map.xy_idx(x, y, 10);
                            let mut neighbors = 0;

                            if self.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx - self.map.width as usize] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + self.map.width as usize] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx - (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx - (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }

                            if neighbors > 4 || neighbors <= 1 {
                                newtiles[idx] = TileType::Wall;
                            }
                            else {
                                newtiles[idx] = TileType::Floor;
                            }
                        }
                    }
                } else {
                    for y in 1..self.map.height - 1 {
                        for x in 1..self.map.width - 1 {
                            let idx = self.map.xy_idx(x, y, 11);
                            let mut neighbors = 0;

                            if self.map.tiles[idx - 1] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + 1] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx - self.map.width as usize] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + self.map.width as usize] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx - (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx - (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + (self.map.width as usize - 1)] == TileType::Wall { neighbors += 1; }
                            if self.map.tiles[idx + (self.map.width as usize + 1)] == TileType::Wall { neighbors += 1; }

                            if neighbors > 4 { //|| neighbors == 0 {
                                newtiles[idx] = TileType::Wall;
                            }
                            else {
                                newtiles[idx] = TileType::Floor;
                            }
                        }
                    }
                }
                self.map.tiles = newtiles.clone();
                self.take_snapshot();
            }

            let mut pos_x;
            let mut pos_y;
            loop {
                //let mut dice = rltk::RandomNumberGenerator::new();
                pos_x = rng.roll_dice(1, self.map.width - 1);
                pos_y = rng.roll_dice(1, self.map.height - 1);
                let idx = self.map.xy_idx(pos_x, pos_y, 12);
                if self.map.tiles[idx] == TileType::Floor {
                    break;
                }
            }
            
            self.flood_fill(pos_x, pos_y);
            let fill_percentage = self.flood_filled.len() as f64 / self.map.tiles.len() as f64;
            //println!("{}", fill_percentage);
            
            if fill_percentage > 0.47 {
                gen_new_map = false;
            }
        }

        // Find a starting point; start at the middle and walk left until we find an open tile
        self.starting_position = Position{ x: self.map.width / 2, y : self.map.height / 2 };
        let mut start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y, 13);
        let mut rng = RandomNumberGenerator::new();
        let roll = rng.roll_dice(1, 4);
        let dir: (i32, i32);
        match roll {
            1 => dir = (-1, 0),
            2 => dir = (1, 0),
            3 => dir = (0, -1),
            _ => dir = (0, 1),
        }
        while self.map.tiles[start_idx] != TileType::Floor {
            self.starting_position.x += dir.0;
            self.starting_position.y += dir.1;
            start_idx = self.map.xy_idx(self.starting_position.x, self.starting_position.y, 14);
        }

        //find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_return_most_distant(&mut self.map, start_idx);
        self.take_snapshot();

        //Place stairs
        self.map.tiles[exit_tile] = TileType::DownStairs;
        self.take_snapshot();

        //Build noise map for use in spawning entities
        self.noise_areas = generate_voronoi_spawn_regions(&self.map, &mut rng);

        // Spawn the entities
        for area in self.noise_areas.iter() {
            spawner::spawn_region(&self.map, &mut rng, area.1, self.depth, &mut self.spawn_list);
        }   
    }

    fn flood_fill(&mut self, x: i32, y: i32) {
        let idx = self.map.xy_idx(x, y, 15);
        if !self.flood_filled.contains(&idx) && self.map.tiles[idx] == TileType::Floor {
            self.flood_filled.push(idx);

            if x > 0 {self.flood_fill(x - 1, y);}
            if x < self.map.width {self.flood_fill(x + 1, y);}
            if y > 0 {self.flood_fill(x, y - 1);}
            if y < self.map.height {self.flood_fill(x, y + 1);}
        }
    }
}