use sekkei::parser::beatmap::{
    HitObject
};

pub struct DifficultyHitObject {
    hit_object: HitObject
}

impl DifficultyHitObject {
    pub fn new(hit_object: HitObject) -> Self {
        DifficultyHitObject {
            hit_object
        }
    }
}