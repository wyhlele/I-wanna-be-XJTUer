use bevy::prelude::*;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

const BEGINX: f32 = 0.;
const BEGINY: f32 = 0.;

const SAVE: i8 = 0;

#[derive(Resource, Debug, Default)]
pub struct KidSaver{
    pub position: Vec2,
    pub save_id: i8,
}

pub struct KidSaverPlugin;

impl Plugin for KidSaverPlugin{
    fn build(&self,app: &mut App){
        app.init_resource::<KidSaver>()
            .add_systems(Startup,create_saver);
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
        if numbers.len() >= 3 {
            *kid_saver = KidSaver{
                position: Vec2::new(numbers[1] as f32, numbers[2] as f32),
                save_id: numbers[0] as i8,
            };
        }

    } else {
        *kid_saver = KidSaver{
            position: Vec2::new(BEGINX, BEGINY),
            save_id: SAVE,
        };
    }
    
}