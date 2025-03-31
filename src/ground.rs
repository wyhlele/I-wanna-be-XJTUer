use std::collections::btree_map::Range;

use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};

pub struct GroundPlugin;

impl Plugin for GroundPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let image = asset_server.load("images/bg.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 9, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    let sprite = Sprite{
        image: image.clone(),
        texture_atlas: Some(atlas.clone()),
        ..Default::default()
    };

    let mut x: f32 = -256.0;
    while x <= 256.0{
        commands.spawn((
            sprite.clone()
        )).insert(
            Transform::from_xyz(x,-64.0,0.0)
        );
        x += 32.0;
    };

    commands.spawn(
        RigidBody::Fixed
    ).insert(
        Transform::from_xyz(0.0,-64.0,0.0)
    ).insert(
        Collider::cuboid(272.0, 16.0)
    );
}