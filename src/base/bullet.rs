use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;

use crate::asset_loader::{ImageAssets, MusicAssets};
use crate::base::kid::Kid;
use crate::base::wrap::spawn_once_warp;
use crate::boss::boss::{Blood, Boss};
use crate::schedule::InGameSet;
use crate::state::{BGMReload, NeedReload, BGM};

#[derive(Component, Debug, Default)]
pub struct Bullet;

pub struct BulletPlugin;

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, spawn_bullet.in_set(InGameSet::UserInput))
        .add_systems(Update, remove_bullet);
    }
}

const BULLET_SPEED:f32 = 500.0;

fn spawn_bullet(
    mut commands: Commands,
    query: Query<(&mut Transform,&mut Kid),With<Kid>>,
    image_assets: Res<ImageAssets>,
    keyboard_input:Res<ButtonInput<KeyCode>>,
    music_assets: Res<MusicAssets>,
){
    if keyboard_input.just_pressed(KeyCode::KeyZ){
        let Ok((transform,kid)) = query.get_single()
        else{
            return ;
        } ;
        let mut vel = Velocity{
            linvel: Vec2 { x: BULLET_SPEED, y: 0.0 },
            angvel: 0.0,
        };
        if kid.state == 1{
            vel.linvel = Vec2 { x: -BULLET_SPEED, y: 0.0 };
        }
        let mut trans = transform.clone();
        trans.scale = Vec3::new(0.5,0.5,0.5);
        commands.spawn((
            Sprite{
                image: image_assets.bullet.clone(),
                ..Default::default()
            },
            Bullet,
        )).insert(
            trans.clone()
        ).insert(
            vel.clone()
        ).insert(
            RigidBody::Dynamic
        ).insert(
            GravityScale(0.0)
        ).insert(
            Ccd::enabled()
        ).insert(
            Collider::ball(2.0)
        ).insert(
            CollisionGroups::new(
                Group::GROUP_4,
                Group::GROUP_2|Group::GROUP_3|Group::GROUP_4,
            )
        ).insert(SolverGroups::new(
            Group::GROUP_4,
            Group::NONE,
            )
        ).insert(ActiveEvents::COLLISION_EVENTS)
        .insert(NeedReload);
        commands.spawn(AudioPlayer::new(music_assets.shoot.clone())).insert(NeedReload);
    }
}

fn remove_bullet(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut boss_query: Query<&mut Boss,(With<Boss>,Without<BGM>,Without<Blood>,Without<Bullet>)>,
    mut blood_query: Query<&mut Sprite,(With<Blood>,Without<Boss>,Without<BGM>,Without<Bullet>)>,
    bgm_query: Query<Entity,(With<BGM>,Without<Boss>,Without<Blood>,Without<Bullet>)>,
    bullet_query: Query<&Bullet,(With<Bullet>,Without<BGM>,Without<Boss>,Without<Blood>)>,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _) => {
                let is_entity1_b = bullet_query.get(*entity_b).is_ok();
                let is_entity2_a = boss_query.get(*entity_a).is_ok();
                if is_entity1_b{
                    commands.entity(*entity_b).despawn_recursive();
                }
                if is_entity1_b && is_entity2_a{
                    let mut boss = boss_query.get_mut(*entity_a).unwrap();
                    if boss.countdown>0{
                        continue;
                    }
                    boss.state -= 1;
                    boss.countdown = 200;
                    for mut sprite in blood_query.iter_mut(){
                        if let Some(atlas) = &mut sprite.texture_atlas{
                            atlas.index = boss.state;
                        }
                    }
                    if boss.state == 0{
                        for bgm in bgm_query.iter(){
                            commands.entity(bgm).despawn_recursive();
                        }
                        
                        commands.entity(*entity_a).despawn_recursive();

                        let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
                        let wr_atlas_layout = texture_atlases.add(wr_layout);
                        let wr_atlas = TextureAtlas{
                            layout : wr_atlas_layout,
                            index : 0,
                        };
                        let wr_image = image_assets.warp.clone();

                        let warp = spawn_once_warp(&mut commands,&wr_image,&wr_atlas,-2400.,608.,-1600.,0.);
                        commands.entity(warp).insert(BGMReload{id:14});
                    }
                }
            }
            _ => {}
        }
    }
}

