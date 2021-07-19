use specs::prelude::*;
use super::{Viewshed, Position,Monster};
use rltk::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
	ReadExpect<'a, Point>,
	ReadStorage<'a, Viewshed>,
	ReadStorage<'a, Monster>);

    fn run(&mut self, data: Self::SystemData) {
	let (player_pos, viewshed, monster) = data;

	for (viewshed, _monster) in (&viewshed, &monster).join() {
	    if viewshed.visible_tiles.contains(&*player_pos) {
		console::log("Monster considers their own existence");
	    }
	}
    }
}
