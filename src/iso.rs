use bevy::prelude::*;

pub const TILE_WIDTH: f32 = 64.0;
pub const TILE_HEIGHT: f32 = 32.0;
pub const TILE_Z_STEP: f32 = 32.0;

pub fn grid_to_world(x: f32, y: f32, z: f32) -> Vec2 {
    Vec2::new(
        (x - y) * (TILE_WIDTH / 2.0),
        -(x + y) * (TILE_HEIGHT / 2.0) + z * TILE_Z_STEP,
    )
}

#[allow(dead_code)]
pub fn world_to_grid(world: Vec2, z: f32) -> Vec2 {
    let wy = world.y - z * TILE_Z_STEP;
    let u = 2.0 * world.x / TILE_WIDTH; // = x - y
    let v = -2.0 * wy / TILE_HEIGHT;    // = x + y
    Vec2::new((u + v) / 2.0, (v - u) / 2.0)
}

pub struct IsoPlugin;

impl Plugin for IsoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_iso_grid);
    }
}

fn draw_iso_grid(mut gizmos: Gizmos) {
    let n = 10;
    let color = Color::srgb(0.25, 0.25, 0.30);

    for x in 0..=n {
        gizmos.line_2d(
            grid_to_world(x as f32, 0.0, 0.0),
            grid_to_world(x as f32, n as f32, 0.0),
            color,
        );
    }

    for y in 0..=n {
        gizmos.line_2d(
            grid_to_world(0.0, y as f32, 0.0),
            grid_to_world(n as f32, y as f32, 0.0),
            color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trip_grid_world_grid() {
        for x in 0..20 {
            for y in 0..20 {
                let (gx, gy) = (x as f32, y as f32);
                let back = world_to_grid(grid_to_world(gx, gy, 0.0), 0.0);
                assert!((back.x - gx).abs() < 1e-4);
                assert!((back.y - gy).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn origin_maps_to_center() {
        assert_eq!(grid_to_world(0.0, 0.0, 0.0), Vec2::ZERO);
    }

    #[test]
    fn higher_z_is_higher_on_screen() {
        assert!(grid_to_world(3.0, 3.0, 1.0).y > grid_to_world(3.0, 3.0, 0.0).y);
    }
}