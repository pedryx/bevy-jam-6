use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn plugin(app: &mut App) {
    app.init_resource::<GlobalRng>();
}

#[derive(Resource)]
pub struct GlobalRng(pub StdRng);

impl FromWorld for GlobalRng {
    fn from_world(_: &mut World) -> Self {
        let seed: u64 = 0xDEAD_BEEF_CAFE_F00D;
        GlobalRng(StdRng::seed_from_u64(seed))
    }
}

impl GlobalRng {
    pub fn next_in_rect(&mut self, rect: Rect) -> Vec2 {
        let x = self.0.gen_range(rect.min.x..rect.max.x);
        let y = self.0.gen_range(rect.min.y..rect.max.y);
        Vec2::new(x, y)
    }
}