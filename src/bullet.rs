use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy::sprite::Sprite;

use crate::asset_loader::ImageAssets;
use crate::kid::Kid;
use crate::schedule::InGameSet;

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
){
    if keyboard_input.just_pressed(KeyCode::KeyZ){
        info!("Z pressed");
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
        ).insert(ActiveEvents::COLLISION_EVENTS);
    }
}

fn remove_bullet(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    bullet_query: Query<&Bullet>,
){
    for collision_event in collision_events.read() {
        // info!("{:?}",collision_event);
        match collision_event {
            CollisionEvent::Started(_, entity_b, _) => {
                let is_entity1_b = bullet_query.get(*entity_b).is_ok();
                if is_entity1_b{
                    commands.entity(*entity_b).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}

