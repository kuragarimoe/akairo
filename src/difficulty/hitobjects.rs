use sekkei::{
    parser::beatmap::{
        objects::HitObject
    },
    util::Vector2
};

pub struct DifficultyHitObject<'hitobj> {
    pub current_hit_object: &'hitobj HitObject,
    pub previous_hit_object: Option<&'hitobj HitObject>,
    pub second_previous_hit_object: Option<&'hitobj HitObject>,
}
