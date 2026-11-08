use horizon_mod_sdk::world;

#[derive(Clone, PartialEq, Eq)]
pub struct Target {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub id: String,
}

pub fn target(distance: f32) -> Option<Target> {
    let hit = world::raycast(distance)?;
    Some(Target {
        x: hit.block_x,
        y: hit.block_y,
        z: hit.block_z,
        id: hit.block,
    })
}
