mod asset_loader;
mod base;
mod boss;
mod building;
mod camera;
mod festival;
mod kid_saver;
mod menu;
mod schedule;
mod state;
mod museum;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use asset_loader::AssetLoaderPlugin;
use base::kid::KidPlugin;
use base::apple::ApplePlugin;
use base::moveto::MovePlugin;
use base::hidden::HiddenPlugin;
use base::wrap::WarpPlugin;
use base::savepointer::SavePointerPlugin;
use base::bullet::BulletPlugin;
use base::toucher::ToucherPlugin;
use boss::boss::BossPlugin;
use building::center::CenterPlugin;
use building::building_a::BuildingAPlugin;
use building::building_b::BuildingBPlugin;
use building::building_c::BuildingCPlugin;
use building::building_d::BuildingDPlugin;
use building::building_e::BuildingEPlugin;
use building::building_top::BuildingTopPlugin;
use camera::CameraPlugin;
use festival::level1::Fest1Plugin;
use festival::level2::Fest2Plugin;
use festival::level3::Fest3Plugin;
use festival::leaf::LeafPlugin;
use kid_saver::KidSaverPlugin;
use menu::startpage::StartPagePlugin;
use menu::achievement::AchievementPagePlugin;
use menu::endpage::EndPagePlugin;
use museum::quiz1::Quiz1Plugin;
use museum::quiz2::Quiz2Plugin;
use museum::quiz3::Quiz3Plugin;
use museum::quiz4::Quiz4Plugin;
use museum::quiz5::Quiz5Plugin;
use schedule::SchedulePlugin;
use state::StatePlugin;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0,1.0,1.0)))
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I Wanna Be XJTUer".into(),
                    resolution: (800., 608.).into(),
                    resizable: false,
                    decorations: true,
                    enabled_buttons: bevy::window::EnabledButtons{
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(KidPlugin)
        .add_plugins(KidSaverPlugin)
        .add_plugins(SavePointerPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(ApplePlugin)
        .add_plugins(HiddenPlugin)
        .add_plugins(MovePlugin)
        .add_plugins(ToucherPlugin)
        .add_plugins(WarpPlugin)
        .add_plugins(StartPagePlugin)
        .add_plugins(EndPagePlugin)
        .add_plugins(LeafPlugin)
        .add_plugins(Fest1Plugin)
        .add_plugins(Fest2Plugin)
        .add_plugins(Fest3Plugin)
        .add_plugins(Quiz1Plugin)
        .add_plugins(Quiz2Plugin)
        .add_plugins(Quiz3Plugin)
        .add_plugins(Quiz4Plugin)
        .add_plugins(Quiz5Plugin)
        .add_plugins(CenterPlugin)
        .add_plugins(BuildingAPlugin)
        .add_plugins(BuildingBPlugin)
        .add_plugins(BuildingCPlugin)
        .add_plugins(BuildingDPlugin)
        .add_plugins(BuildingEPlugin)
        .add_plugins(BuildingTopPlugin)
        .add_plugins(BossPlugin)
        .add_plugins(AchievementPagePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}


