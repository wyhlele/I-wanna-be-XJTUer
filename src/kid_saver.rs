use bevy::prelude::*;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use crate::asset_loader::{AchievementAssets, ImageAssets};
use crate::base::wrap::spawn_single_warp;

const BASEX: f32 = 0.0;
const BASEY: f32 = 608.0;

const POS: [bevy::prelude::Vec2; 15] = [
    Vec2::new(0.,0.),
    Vec2::new(800.-9.*32.,608.-8.*32.),
    Vec2::new(0.-11.*32.,2.*608.-8.*32.),
    Vec2::new(-800.-10.*32.,608.+3.*32.),
    Vec2::new(-2.*800.,-2.*608.),
    Vec2::new(-800.,-608.),
    Vec2::new(0.,-2.*608.),
    Vec2::new(800.,-608.),
    Vec2::new(2.*800.,-2.*608.),
    Vec2::new(2.*800.,2.*608.+2.*32.),
    Vec2::new(2.*800.-8.*32.,-8.*32.),
    Vec2::new(2.*800.+3.*32.,2.*32.),
    Vec2::new(2.*800.+11.*32.,5.*32.),
    Vec2::new(-3.*800.,608.),
    Vec2::new(-2.*800.,0.),
];

const SAVE: i8 = 0;

#[derive(Resource, Debug, Default)]
pub struct KidSaver{
    pub position: Vec2,
    pub achi: i16,
    pub solve: i8,
    pub save_id: i8,
}

pub struct KidSaverPlugin;

impl Plugin for KidSaverPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<KidSaver>()
            .add_systems(Startup,create_saver)
            .add_systems(PostStartup,draw_achi);
    }
}

fn create_saver(
    mut kid_saver: ResMut<KidSaver>,
){
    let file_path = Path::new("save");
    if file_path.exists() {
        let file = fs::File::open(file_path).expect("ERROR: cannot open file save");
        let reader = io::BufReader::new(file);

        let mut numbers = Vec::new();
        for line in reader.lines() {
            let line = line.expect("ERROR: cannot open file save");
            for word in line.split_whitespace() {
                if let Ok(num) = word.parse::<i32>() {
                    numbers.push(num);
                }
            }
        }
        if numbers.len() == 3 {
            if numbers[0] == 13 || numbers[0]>14 || numbers[0]<0{
                numbers[0] = 0;
                numbers[1] = 0;
                numbers[2] = 0;
                warn!("File save damaged. Reload the game.");
            }
            *kid_saver = KidSaver{
                position: POS[numbers[0] as usize],
                achi: numbers[1] as i16,
                solve: numbers[2] as i8,
                save_id: numbers[0] as i8,
            };
        }else{
            warn!("File save damaged. Reload the game.");
            *kid_saver = KidSaver{
                position: POS[SAVE as usize],
                achi: 0,
                solve: 0,
                save_id: SAVE,
            };
        }
    } else {
        *kid_saver = KidSaver{
            position: POS[SAVE as usize],
            achi: 0,
            solve: 0,
            save_id: SAVE,
        };
    }

    
}

fn draw_achi(
    mut commands: Commands,
    achievement_assets: Res<AchievementAssets>,
    kid_saver: Res<KidSaver>,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>
){
    for i in 0..10{
        if (kid_saver.achi>>i)&1==1{
            let image;
            let pos;
            match i {
                0 =>{
                    image = achievement_assets.achievement0.clone();
                    pos = Vec3::new(BASEX+-256.,BASEY+144.,0.7);
                },
                1 =>{
                    image = achievement_assets.achievement1.clone();
                    pos = Vec3::new(BASEX+0.,BASEY+144.,0.7);
                },
                2 =>{
                    image = achievement_assets.achievement2.clone();
                    pos = Vec3::new(BASEX+256.,BASEY+144.,0.7);
                },
                3 =>{
                    image = achievement_assets.achievement3.clone();
                    pos = Vec3::new(BASEX-256.,BASEY+32.,0.7);
                },
                4 =>{
                    image = achievement_assets.achievement4.clone();
                    pos = Vec3::new(BASEX+0.,BASEY+32.,0.7);
                },
                5 =>{
                    image = achievement_assets.achievement5.clone();
                    pos = Vec3::new(BASEX+256.,BASEY+32.,0.7);
                },
                6 =>{
                    image = achievement_assets.achievement6.clone();
                    pos = Vec3::new(BASEX-256.,BASEY-80.,0.7);
                },
                7 =>{
                    image = achievement_assets.achievement7.clone();
                    pos = Vec3::new(BASEX+0.,BASEY-80.,0.7);
                },
                8 =>{
                    image = achievement_assets.achievement8.clone();
                    pos = Vec3::new(BASEX+256.,BASEY-80.,0.7);
                },
                _ =>{
                    image = achievement_assets.achievement9.clone();
                        pos = Vec3::new(BASEX+0.,BASEY-192.,0.8);
                    },
            }
            commands.spawn(
                Sprite{
                    image: image,
                    ..Default::default()
                }
            ).insert(Transform{
                translation: pos,
                scale: Vec3::new(0.8,0.8,1.),
                ..Default::default()
            });
        }
    }
    if kid_saver.achi == 1023{
        let wr_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None);
        let wr_atlas_layout = texture_atlases.add(wr_layout);
        let wr_atlas = TextureAtlas{
            layout : wr_atlas_layout,
            index : 0,
        };
        let wr_image = image_assets.warp.clone();

        spawn_single_warp(&mut commands,&wr_image,&wr_atlas,-1600.-384.,-256.,BASEX,BASEY-256.);
        spawn_single_warp(&mut commands,&wr_image,&wr_atlas,BASEX-384.,BASEY-256.,-1600.,-256.);
    }
        
}