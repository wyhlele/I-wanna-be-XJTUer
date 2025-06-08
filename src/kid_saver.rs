use bevy::prelude::*;

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

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
    Vec2::new(-2.*800.,-8.*32.),
];

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
        if numbers.len() >= 1 {
            if numbers[0] == 13 || numbers[0]>14{
                numbers[0] = 0;
                warn!("File save damaged. Reload the game.");
            }
            *kid_saver = KidSaver{
                position: POS[numbers[0] as usize],
                save_id: numbers[0] as i8,
            };
        }

    } else {
        *kid_saver = KidSaver{
            position: POS[SAVE as usize],
            save_id: SAVE,
        };
    }
    
}