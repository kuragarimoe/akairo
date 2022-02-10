use sekkei::{
    parser::beatmap::{
        objects::HitObject
    },
    util::Vector2
};

pub struct DifficultyHitObject {
    pub current_hit_object: Option<HitObject>,
    pub previous_hit_object: Option<HitObject>,
    pub second_previous_hit_object: Option<HitObject>,
}