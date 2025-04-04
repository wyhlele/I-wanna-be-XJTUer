
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

use bevy::sprite::{TextureAtlas, TextureAtlasLayout,Sprite};

use crate::asset_loader::ImageAssets;

pub struct GroundPlugin;

impl Plugin for GroundPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup,spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){
    let image = image_assets.bg.clone();
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
        commands.spawn(sprite.clone()).insert(
            Transform::from_xyz(x,-64.0,0.0)
        );
        x += 32.0;
    };
    commands.spawn(sprite.clone()).insert(
        Transform::from_xyz(-256.0,-32.0,0.0)
    );
    

    commands.spawn(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(0.0,-64.0,0.0)
    ).insert(
        Collider::cuboid(272.0, 16.0)
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_2,
        Group::GROUP_1,
    ));


    commands.spawn(
        RigidBody::Fixed
    ).insert(
        GravityScale(0.0)
    ).insert(
        Transform::from_xyz(-256.0,-32.0,0.0)
    ).insert(
        Collider::cuboid(16.0, 16.0)
    ).insert(
        LockedAxes::ROTATION_LOCKED
    ).insert(
        CollisionGroups::new(
            Group::GROUP_2,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_2,
        Group::GROUP_1,
    ));
}