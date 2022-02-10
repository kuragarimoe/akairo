use sekkei::{
    game::Mods,
    parser::beatmap::{
        BeatmapFile,
        objects::{
            HitObject
        }
    },
    util::Vector2
};

use crate::{
    difficulty::hitobjects::{
        DifficultyHitObject
    },
    difficulty::standard::skills::{
        AimSkill
    }
};

pub mod skills;

pub struct StandardDifficultyCalculator {
    // internal values
    pub time_rate: f64,

    // difficulty hit object vlaues
    pub travel_distance: f32,
    pub jump_distance: f32,
    pub angle: f32,
    pub delta_time: f32,
    pub strain_time: f32
}

impl StandardDifficultyCalculator {
    fn calculate(&mut self, beatmap: &BeatmapFile, mods: i32) -> Option<StandardResult> {
        // constants
        let section_number = 400;
        let difficulty_multiplier = 0.0675;

        // get time rate
        if mods & Mods::DoubleTime as i32 != 0 {
            self.time_rate = 1.5;
        } else if mods & Mods::HalfTime as i32 != 0 {
            self.time_rate = 0.75;
        } else {
            self.time_rate = 1.0;
        }

        // create skills
        //self.create_skills(mods);
        self.create_hit_objects(beatmap);


        None
    }

    /*fn create_skills(&self, mods: i32) -> Vec {
        let skill_vec = vec![];
    }*/

    fn create_hit_objects<'beatmap>(&self, beatmap: &'beatmap BeatmapFile) -> Vec<DifficultyHitObject<'beatmap>> {
        // constants
        let normalized_radius = 52.0;
        let mut diff_hit_objects = Vec::new();

        for (i, obj) in beatmap.hit_objects.iter().enumerate() {
            let scaling_factor = 52.0;

            diff_hit_objects.push(DifficultyHitObject {
                current_hit_object: obj,
                previous_hit_object: beatmap.hit_objects.get(i - 1),
                second_previous_hit_object: beatmap.hit_objects.get(i - 2),
            })

        }

        diff_hit_objects
    }

    /*fn create_hit_object(&self, current_hit_object: HitObject, previous_hit_object: HitObject, second_previous_hit_object: HitObject) -> DifficultyHitObject {
        DifficultyHitObject {
            current_hit_object,
            position: Vector2::new(hit_object.x, hit_object.y)
        }
    }*/

}

pub struct StandardResult {

}
