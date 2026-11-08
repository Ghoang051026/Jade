use horizon_mod_sdk::world;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn target(distance: f64) -> Option<Target> {
    let hit = world::raycast(distance)?;
    Some(Target {
        x: hit.block_x as i32,
        y: hit.block_y as i32,
        z: hit.block_z as i32,
    })
}

pub fn block_id(t: Target) -> Option<String> {
    world::get_block(t.x, t.y, t.z)
}

pub fn biome(t: Target) -> Option<String> {
    world::get_biome(t.x, t.y, t.z)
}
