use bevy::prelude::*;

use crate::grid::{ClearSelectionsEvent, Selection, Tile};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_wall);
    }
}

#[derive(Component)]
struct Wall;

fn spawn_wall(
    mut commands: Commands,
    mut q_selection: Query<(Entity, &mut Tile), With<Selection>>,
    keyboard: Res<Input<KeyCode>>,
    mut ev_clear: EventWriter<ClearSelectionsEvent>,
) {
    if keyboard.just_pressed(KeyCode::W) {
        ev_clear.send(ClearSelectionsEvent);
        for (ent, mut tile) in q_selection.iter_mut() {
            tile.cost = 200;
            commands.entity(ent).with_children(|commands| {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BEIGE,
                            custom_size: Some(Vec2::splat(26.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                        ..default()
                    })
                    .insert(Wall);
            });
        }
    }
}
