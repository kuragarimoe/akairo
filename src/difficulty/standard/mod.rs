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
    pub beatmap: BeatmapFile,
    pub time_rate: f64,
    
    // difficulty hit object vlaues
    pub travel_distance: f32,
    pub jump_distance: f32,
    pub angle: f32,
    pub delta_time: f32,
    pub strain_time: f32
}

impl StandardDifficultyCalculator {
    fn calculate(&mut self, beatmap: BeatmapFile, mods: i32) -> Option<StandardResult> {
        self.beatmap = beatmap;

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
        self.create_skills(mods);


        None
    }

    /*fn create_skills(&self, mods: i32) -> Vec {
        let skill_vec = vec![];
    }*/

    fn create_hit_objects(self, hit_objects: Vec<HitObject>) -> Vec<DifficultyHitObject> {
        // constants
        let normalized_radius = 52.0;
        let diff_hit_objects = vec![];

        let mut previous_hit_object: Option<DifficultyHitObject> = None;

        for i in 0..self.beatmap.hit_objects.len() {
            let hit_object = &self.beatmap.hit_objects[i];
            let scalingFactor = (52.0);

            diff_hit_objects.push(DifficultyHitObject {
                current_hit_object: self.beatmap.hit_objects.get(i).clone(),
                previous_hit_object: self.beatmap.hit_objects.clone().get(i - 1),
                second_previous_hit_object: self.beatmap.hit_objects.clone().get(i - 1),
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