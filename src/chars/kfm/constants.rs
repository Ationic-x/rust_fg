/// Atributos propios del personaje KFM
pub mod constants {
    // [Filenames]
    pub const SFF: &str = "kfm";
    pub const CMD: &str = "kfm";
    pub const AIR: &str = "kfm";
    

    // [Data]
    pub const LIFE: i32 = 1000;
    pub const POWER: u32 = 3000;
    // pub const ATTACK: i32 = 100;
    // pub const DEFENCE: i32 = 100;
    // pub const FALL_DEFENCE_UP: i32 = 50;
    // pub const LIE_DOWN_TIME: i32 = 60;
    // pub const AIR_JUGGLE: i32 = 15;
    // pub const SPARK_NO: i32 = 2;
    // pub const GUARD_SPARK_NO: i32 = 40;
    // pub const KO_ECHO: i32 = 0;
    // pub const VOLUME: i32 = 0;
    // pub const INT_PERSIST_INDEX: i32 = 60;
    // pub const FLOAT_PERSIST_INDEX: i32 = 40;
    pub const MAXIMUM_NUMBER_JUMPS: i32 = 2;

    // [Size]
    // pub const XSCALE: f64 = 1.0;
    // pub const YSCALE: f64 = 1.0;
    // pub const GROUND_BACK: i32 = 15;
    // pub const GROUND_FRONT: i32 = 16;
    // pub const AIR_BACK: i32 = 12;
    // pub const AIR_FRONT: i32 = 12;
    // pub const HEIGHT: i32 = 60;
    // pub const ATTACK_DIST: i32 = 160;
    // pub const PROJ_ATTACK_DIST: i32 = 90;
    // pub const PROJ_DOSCALE: i32 = 0;
    // pub const HEAD_POS: (i32, i32) = (-5, -90);
    // pub const MID_POS: (i32, i32) = (-5, -60);
    // pub const SHADOWOFFSET: i32 = 0;
    // pub const DRAW_OFFSET: (i32, i32) = (0, 0);

    // [Velocity]
    pub const WALK_FWD: f64 = 2.4;
    pub const WALK_BACK: f64 = -2.2;
    pub const RUN_FWD: f64 = 4.6;
    pub const RUN_BACK: (f64, f64) = (-4.5, -3.8);
    pub const JUMP_Y: f64 = -8.4;
    pub const JUMP_BACK_X: f64 = -2.55;
    pub const JUMP_FWD_X: f64 = 2.5;
    pub const RUNJUMP_BACK: (f64, f64) = (-2.55, -4.1);
    // pub const RUNJUMP_FWD: (f64, f64) = (4.0, -8.1);
    // pub const AIRJUMP_NEU: i32 = 1;
    // pub const AIRJUMP_HEIGHT: i32 = 35;
    // pub const YACCEL: f64 = 0.44;
    // pub const STAND_FRICTION: f64 = 0.85;
    pub const CROUCH_FRICTION: f64 = 0.82;
    // pub const STAND_FRICTION_THRESHOLD: f64 = 2.0;
    pub const CROUCH_FRICTION_THRESHOLD: f64 = 0.05;
    // pub const AIRGETHIT_GROUNDLEVEL: i32 = 25;
    // pub const AIRGETHIT_GROUNDRECOVER_GROUND_THRESHOLD: i32 = -20;
    // pub const AIRGETHIT_GROUNDRECOVER_GROUNDLEVEL: i32 = 10;
    // pub const AIRGETHIT_AIRRECOVER_THRESHOLD: i32 = -1;
    // pub const AIRGETHIT_AIRRECOVER_YACCEL: f64 = 0.35;
    // pub const AIRGETHIT_TRIP_GROUNDLEVEL: i32 = 15;
    // pub const DOWN_BOUNCE_OFFSET: (i32, i32) = (0, 20);
    // pub const DOWN_BOUNCE_YACCEL: f64 = 0.4;
    // pub const DOWN_BOUNCE_GROUNDLEVEL: i32 = 12;
    // pub const DOWN_FRICTION_THRESHOLD: f64 = 0.05;

    // [Quotes]
    // pub const VICTORY1: &str = "You must defeat Tu Shou to stand a chance.";
    // pub const VICTORY2: &str = "You need a lot of training. Come back in ten years.";
    // pub const VICTORY3: &str = "You look familiar. Did I see you on TV?";
    // pub const VICTORY4: &str = "Your moves are too predictable. You want to learn Kung Fu Palm? It's not for novices.";
    // pub const VICTORY5: &str = "That was a good workout.";
    // pub const VICTORY6: &str = "I want to challenge more skilled fighters.";
    // pub const VICTORY7: &str = "What do you mean my girlfriend is in another temple?";
}