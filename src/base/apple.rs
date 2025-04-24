use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;

use crate::asset_loader::ImageAssets;
use crate::schedule::InGameSet;
use crate::base::trap::Trap;

pub struct ApplePlugin;

#[derive(Component, Debug, Default)]
pub struct Apple;

impl Plugin for ApplePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PostStartup, spawn_apple)
        .add_systems(Update, update_apple.in_set(InGameSet::EntityUpdates));
    
    }
}

#[derive(Resource)]
pub struct AnimationTimer(Timer);

pub fn spawn_single_apple(
    commands: &mut Commands,
    sprtie: &Handle<Image>,
    atlas: &TextureAtlas,
    x: f32,y: f32,
) -> Entity{
    commands.spawn((
        Sprite{
            image: sprtie.clone(),
            texture_atlas: Some(atlas.clone()),
            ..Default::default()
        },
        Trap,
        Apple,
    )).insert(
        Transform::from_xyz(x,y,0.0)
    ).insert(
        Velocity::zero()
    ).insert(
        RigidBody::Dynamic
    ).insert(
        GravityScale(0.0)
    ).insert(
        Collider::ball(10.0)
    ).insert(
        Ccd::enabled()
    ).insert(
        CollisionGroups::new(
            Group::GROUP_3,
            Group::GROUP_1|Group::GROUP_4,
        )
    ).insert(SolverGroups::new(
        Group::GROUP_3,
        Group::GROUP_1,
        )
    ).id()
}

fn spawn_apple(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
){

    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 2,1, None, None);
    let atlas_layout = texture_atlases.add(layout);
    let atlas = TextureAtlas{
        layout : atlas_layout,
        index : 0,
    };
    let image = image_assets.apple.clone();
    // spawn_single_apple(&mut commands, &image, &atlas, 0.0, 64.0);

    commands.insert_resource(AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn update_apple(
    mut query: Query<&mut Sprite,With<Apple>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
){
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for mut sprite in query.iter_mut(){
            if let Some(atlas) = &mut sprite.texture_atlas{
                atlas.index = (atlas.index + 1) % 2;
            }
        }
    }
}
