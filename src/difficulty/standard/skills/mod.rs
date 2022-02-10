use crate::difficulty::hitobjects::DifficultyHitObject;

/// AIM SKILL
pub struct AimSkill {
    pub base_skill: Skill,
}

impl AimSkill {

}

/// BASE SKILL

pub struct Skill {
    pub current_strain: f32,
    pub strain_peaks: Vec<f32>
}

impl Default for Skill {
    fn default() -> Self {
        Skill {
            current_strain: 0.0,
            strain_peaks: vec![]
        }
    }
}

impl Skill {
    fn process_hit_object(&self, hit_object: &DifficultyHitObject) {
        
    }

    /*fn difficulty_value(&self, mods: i32) -> f32 {
        &self.strain_peaks.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

        let mut value = 0.0;
        let weight = 1.0;

        for peak in &self.strain_peaks {
            value += (peak as f32) * weight;
            weight *= 0.95;
        }

        value
    }

    fn strain_decay(&self) -> None {
        None
    }*/
}