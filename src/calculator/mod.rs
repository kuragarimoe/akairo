use sekkei::{
    game::{
        Mods
    },
    parser::beatmap::{
        BeatmapFile
    }
};

trait DifficultyCalculator {
    fn calculate_stars(map: BeatmapFile, mods: Mods) -> i32 {
        0
    }
}

trait PerformanceCalculator {

}