use super::{BlocksVisibility, GameLog, Hidden, Map, Name, Player, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

#[allow(clippy::type_complexity)]
impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, BlocksVisibility>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            entities,
            mut viewshed,
            pos,
            player,
            mut hidden,
            mut rng,
            mut log,
            names,
            blocks_visibility,
        ) = data;

        map.view_blocked.clear();
        for (block_pos, _block) in (&pos, &blocks_visibility).join() {
            let idx = map.xy_idx(block_pos.x, block_pos.y);
            map.view_blocked.insert(idx);
        }

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                //Ne retiens que les tiles visibles, rejetant celles hors champ.
                viewshed
                    .visible_tiles
                    .retain(|p| p.x > 0 && p.x < map.width - 1 && p.y > 0 && p.y < map.height - 1);

                //Revèle ce que le joueur voit si l'entité sèl. est le joueur
                let p: Option<&Player> = player.get(ent);
                if let Some(_p) = p {
                    //true seulement si il y a un composant joueur
                    //Remet toute les cases "visible" en faux
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        //Redondant avec .retains plus haut
                        if vis.x > 0 && vis.x < map.width - 1 && vis.y > 0 && vis.y < map.height - 1
                        {
                            let idx = map.xy_idx(vis.x, vis.y);
                            map.revealed_tiles[idx] = true;
                            map.visible_tiles[idx] = true;

                            //Chance de révéler des pièges
                            //A modifier plus tard...
                            for e in map.tile_content[idx].iter() {
                                let maybe_hidden = hidden.get(*e);
                                if let Some(_maybe_hidden) = maybe_hidden {
                                    if rng.roll_dice(1, 24) == 1 {
                                        let name = names.get(*e);
                                        if let Some(name) = name {
                                            log.entries.insert(
                                                0,
                                                format!("You spotted a {}.", &name.name),
                                            );
                                        }
                                        hidden.remove(*e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
