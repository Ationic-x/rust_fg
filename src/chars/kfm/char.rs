use crate::chars::Character;

use super::{constants::constants::*, CharData, State};

pub fn trigger(char: &mut CharData) {
    if char.ctrl {
        direction_hanlder(char);
    }
    if char.state_no != -1 {
        call_state(char);
        return;
    }
    if char.lose {
        char.state_no = 170;
        call_state(char);
    }
    if char.win {
        char.state_no = 181;
        call_state(char);
    }
    if char.action == "" {
        if (char.distance > 0.0) != char.current_flip {
            if char.state == State::S {
                char.state_no = 5;
                call_state(char);
            }
            if char.state == State::C {
                char.state_no = 6;
                call_state(char);
            }
        }
        return;
    }

    if char.action == "FF_ab" && char.power >= 330 {
        if combo_condition_check(char) {
            char.state_no = 1070;
        }
        return;
    }

    if char.action == "SmashKFUpper" && char.state != State::A && char.power >= 1000 {
        if char.ctrl && char.state != State::A {
            char.state_no = 3050;
        }
        return;
    }

    if char.action == "TripleKFPalm" && char.power >= 1000 {
        if char.state == State::S && char.ctrl {
            char.state_no = 3000;
        }
        return;
    }

    if char.action == "FF_a" {
        if combo_condition_check(char) {
            char.state_no = 1050;
        }
        return;
    }

    if char.action == "FF_b" {
        if combo_condition_check(char) {
            char.state_no = 1060;
        }
        return;
    }

    if char.action == "QCF_xy" && char.power >= 330 {
        if combo_condition_check(char) {
            char.state_no = 1020;
        }
        return;
    }

    if char.action == "QCF_x" {
        if combo_condition_check(char) {
            char.state_no = 1000;
        }
        return;
    }

    if char.action == "QCF_y" {
        if combo_condition_check(char) {
            char.state_no = 1010;
        }
        return;
    }

    if char.action == "upper_xy" && char.power >= 330 {
        if combo_condition_check(char) {
            char.state_no = 1120;
        }
        return;
    }

    if char.action == "upper_x" {
        if combo_condition_check(char) {
            char.state_no = 1100;
        }
        return;
    }

    if char.action == "upper_y" {
        if combo_condition_check(char) {
            char.state_no = 1110;
        }
        return;
    }

    if char.action == "QCB_xy" && char.power >= 330 {
        if combo_condition_check(char) {
            char.state_no = 1220;
        }
        return;
    }

    if char.action == "QCB_x" {
        if combo_condition_check(char) {
            char.state_no = 1200;
        }
        return;
    }

    if char.action == "QCB_y" {
        if combo_condition_check(char) {
            char.state_no = 1210;
        }
        return;
    }

    // IMPROVE -----
    // Stand
    if char.action == "blocking" {
        if !char.ctrl {
            return;
        }
        if char.state == State::S {
            char.state_no = 1300;
        }

        if char.state == State::C {
            char.state_no = 1320;
        }
        if char.state == State::A && (char.state_no == 1350) && char.time > 0 {
            char.state_no = 1340;
        }
        return;
    }

    if char.action == "QCF_ab" && char.power >= 330 {
        if combo_condition_check(char) {
            char.state_no = 1420;
        }
        return;
    }

    if char.action == "QCF_a" {
        if combo_condition_check(char) {
            char.state_no = 1400;
        }
        return;
    }

    if char.action == "QCF_b" {
        if combo_condition_check(char) {
            char.state_no = 1410;
        }
        return;
    }

    // Stand Light Punch
    if char.action == "x" {
        if char.state == State::S && char.ctrl {
            char.state_no = 200;
        }
        if char.state == State::C && char.ctrl {
            char.state_no = 400;
        }
        return;
    }

    // Stand Strong Punch
    if char.action == "y" {
        if char.state == State::S && char.ctrl {
            if char.direction == 6 && char.distance.abs() < 32.0 {
                char.state_no = 800;
            } else {
                char.state_no = 210;
            }
        }
        if char.state == State::C && char.ctrl {
            char.state_no = 410;
        }
        return;
    }

    // Stand Light Kick
    if char.action == "a" {
        if char.state == State::S && char.ctrl {
            char.state_no = 230;
        }
        if char.state == State::C && char.ctrl {
            char.state_no = 430;
        }
        return;
    }

    // Standing Strong Kick
    if char.action == "b" {
        if char.state == State::S && char.ctrl {
            char.state_no = 240;
        }
        if char.state == State::C && char.ctrl {
            char.state_no = 440;
        }
        return;
    }

    // Taunt
    if char.action == "start" {
        if char.state != State::A && char.ctrl {
            char.state_no = 195;
        }
        return;
    }
}

fn combo_condition_check(char: &mut CharData) -> bool {
    (char.state != State::A && char.ctrl)
        || (((char.state_no >= 200 && char.state_no <= 299)
            || (char.state_no >= 400 && char.state_no <= 499))
            && char.state_no != 440
            && char.move_contact)
        || (char.state_no == 1310 || char.state_no == 1330)
}

fn direction_hanlder(char: &mut CharData) {
    match char.direction {
        66 => {
            if char.state == State::S {
                char.run = true;
            }
        }
        44 => {
            char.run = false;
            if char.state == State::S {
                char.state_no = 105;
            }
        }
        9 | 7 | 8 => {
            char.run = false;
            if char.state == State::A && char.double_jump && char.jumps < MAXIMUM_NUMBER_JUMPS {
                char.state_no = 40;
                char.anim_time = 0;
                char.new_anim = true;
                char.jumps += 1;
            }
            if char.state == State::S {
                char.state_no = 40;
                char.new_anim = true;
                char.jumps = 1;
            }
            char.double_jump = false;
        }
        6 => {
            if char.state == State::A {
                char.double_jump = true;
                return;
            }
            if char.anim != 20 && char.anim != 100 {
                char.new_anim = true;
            }
            if char.state == State::C {
                char.ctrl = false;
                char.state_no = 12;
            } else {
                if char.run {
                    char.anim = 100;
                    char.set_vel_x(RUN_FWD);
                } else {
                    char.anim = 20;
                    char.set_vel_x(WALK_FWD);
                }
            }
        }
        4 => {
            char.run = false;
            if char.state == State::A {
                char.double_jump = true;
                return;
            }
            if char.anim != 21 {
                char.new_anim = true;
            }
            if char.state == State::C {
                char.ctrl = false;
                char.state_no = 12;
            } else {
                char.anim = 21;
                char.set_vel_x(WALK_BACK);
            }
        }
        5 => {
            char.run = false;
            if char.state == State::A {
                char.double_jump = true;
                return;
            }
            if char.anim != 0 {
                char.new_anim = true;
            }
            if char.state == State::C {
                char.ctrl = false;
                char.state_no = 12;
            }
            if char.state == State::S {
                char.anim = 0;
            }
        }
        1 | 2 | 3 => {
            char.run = false;
            if char.state == State::A {
                char.double_jump = true;
                return;
            }
            if char.anim != 11 {
                char.new_anim = true;
            }
            if char.state == State::S {
                char.ctrl = false;
                char.state_no = 10;
            } else {
                char.anim = 11;
                char.anim_time = -1;
            }
        }
        _ => {
            //char.anim = 0;
        }
    }
}

fn default_end_action(char: &mut CharData) {
    char.flip_x = false;
    char.state_no = -1;
    char.action = "".to_string();
    char.ctrl = true;
    char.fall = false;
    char.run = false;
    trigger(char);
}

pub fn hit_handler(char: &mut CharData, target: &mut dyn Character) {
    let target_state_no = target.get_state_no();
    if (target_state_no == 1300 || target_state_no == 1320 || target_state_no == 1340)
        && target.get_anim_elem() < 2
    {
        return;
    }
    let mut state_no = 5000;
    let mut low_to_high = 0;
    let mut target_blocking = false;
    target.set_def(false);
    if (target.get_distance() > 0.0) != target.get_flip() {
        target.set_current_flip(target.get_distance() > 0.0);
    }
    let target_state = target.get_state();
    let target_direction = target.get_direction();
    if target_state == &State::L {
        return;
    }
    if char.attack != State::C
        && (target_direction == 4
            || target_direction == 7
            || (target_direction == 1 && target_state == &State::A))
        || char.attack != State::A && target_direction == 1
    {
        target_blocking = true;
        state_no = 1300;
        if target_state == &State::S {
            state_no += 10;
        }
        if target_state == &State::C {
            state_no += 30;
        }
        if target_state == &State::A {
            state_no += 50;
        }
    } else {
    }
    if !target_blocking {
        if target_state == &State::C {
            low_to_high = 10;
            state_no += 20;
        }
    }
    match char.state_no {
        200 => {
            if target_blocking {
                if target_state == &State::A {
                    target.set_vel_y(-0.8);
                    target.set_vel_x(-1.9);
                } else {
                    // Original 4
                    target.set_vel_x(-2.0);
                }
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            if target_state == &State::A {
                target.set_vel_y(-0.8);
            }
            target.add_life(-23);
            // Original 4
            target.set_vel_x(-2.0);
            target.set_state_no(state_no);
        }
        210 => {
            if target_state == &State::A {
                // Original 2.5
                target.set_vel_x(-2.0);
                target.set_vel_y(-4.0);
            }
            // Original 5.5
            target.set_vel_x(-2.7);
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-57);
            target.set_state_no(state_no + 1);
        }
        230 => {
            if target_state == &State::A {
                // Original 2.5
                target.set_vel_x(-2.0);
                target.set_vel_y(-3.5);
            }
            // Original 5.0
            target.set_vel_x(-2.5);
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-26);
            target.set_state_no(state_no + 11 - low_to_high);
        }
        240 => {
            if target_state == &State::A {
                // Original 2.2
                target.set_vel_x(-1.7);
                target.set_vel_y(-3.2);
            }
            // Original 6.0
            target.set_vel_x(-3.0);
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-63);
            target.set_state_no(state_no + 11 - low_to_high);
        }
        400 => {
            if target_state == &State::A {
                // Original -1.5
                target.set_vel_x(-1.0);
                target.set_vel_y(-3.0);
            }
            // Original -4
            target.set_vel_x(-2.0);
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-23);
            target.set_state_no(state_no + 10 - low_to_high);
        }
        410 => {
            if target_state == &State::A {
                // Original -3
                target.set_vel_x(-2.0);
                target.set_vel_y(-4.0);
            }
            // Original 7.0
            target.set_vel_x(-3.5);
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-37);
            target.set_state_no(state_no + 11 - low_to_high);
        }
        430 => {
            if target_state == &State::A {
                // Original 2
                target.set_vel_x(-1.7);
                target.set_vel_y(-3.0);
            }
            // Original 5.0
            target.set_vel_x(-2.5);
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-28);
            target.set_state_no(state_no + 10 - low_to_high);
        }
        440 => {
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-72);
            target.set_state_no(state_no + 70 - low_to_high * 2);
        }
        600 => {
            if target_state == &State::A {
                target.set_vel_x(-1.3);
                target.set_vel_y(-3.0);
            } else {
                // Original 4
                target.set_vel_x(-2.0);
            }
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-20);
            target.set_state_no(state_no);
        }
        610 => {
            if target_state == &State::A {
                // Original 3
                target.set_vel_x(-2.5);
                target.set_vel_y(-4.0);
            } else {
                // Original 6
                target.set_vel_x(-3.0);
            }
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-72);
            target.set_state_no(state_no + 1);
        }
        630 => {
            if target_state == &State::A {
                //Orignal 2
                target.set_vel_x(-1.5);
                target.set_vel_y(-3.0);
            } else {
                // Original 4
                target.set_vel_x(-2.0);
            }
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-26);
            target.set_state_no(state_no);
        }
        640 => {
            if target_state == &State::A {
                // Original 3
                target.set_vel_x(-2.5);
                target.set_vel_y(-4.0);
            } else {
                // Original 7
                target.set_vel_x(-3.5);
            }
            if target_blocking {
                target.set_def(true);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-70);
            target.set_state_no(state_no + 1);
        }
        800 => {
            char.add_pos_x(-30.0);
            target.set_x(char.x + if char.is_flipped() { -30.0 } else { 30.0 });
            char.state_no = 810;
            target.add_life(-78);
            target.set_state_no(820);
        }
        1000 => {
            let char_distance = char.distance.abs();
            if target_state == &State::A {
                // Original 4
                target.set_vel_x(-2.0);
                if char_distance < 55.0 {
                    target.add_life(-90);
                    target.set_fall(true);
                    // Original 4
                    target.set_vel_y(-3.5);
                } else {
                    target.add_life(-85);
                    // Original 7
                    target.set_vel_x(-2.5);
                }
            } else {
                if char_distance < 55.0 {
                    target.add_life(-90);
                    target.set_fall(true);
                    target.set_state(State::A);
                    // Original 4, 2.5
                    target.set_vel_x(-4.0);
                    target.set_vel_y(-1.5);
                } else {
                    target.add_life(-85);
                    // Original 7
                    target.set_vel_x(-3.5);
                }
            }
            if target_blocking {
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1010 => {
            let char_distance = char.distance.abs();
            if target_state == &State::A {
                // Original 4
                target.set_vel_x(-2.0);
                if char_distance < 55.0 {
                    target.add_life(-90);
                    target.set_fall(true);
                    // Original 4
                    target.set_vel_y(-3.5);
                } else {
                    target.add_life(-85);
                    // Original 7
                    target.set_vel_x(-2.5);
                }
            } else {
                if char_distance < 55.0 {
                    target.add_life(-90);
                    target.set_fall(true);
                    target.set_state(State::A);
                    // Original 4, 2.5
                    target.set_vel_x(-4.0);
                    target.set_vel_y(-1.5);
                } else {
                    target.add_life(-85);
                    // Original 7
                    target.set_vel_x(-3.5);
                }
            }
            if target_blocking {
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1020 => {
            if target_blocking {
                if target_state == &State::A {
                    // Orginal -7
                    target.set_vel_y(-4.0);
                }
                // Original 8
                target.set_vel_x(-4.0);
                target.set_def(true);
                target.add_life(-5);
                target.set_state_no(state_no);
                return;
            }
            // Original 8, -7
            target.set_vel_x(-4.0);
            target.set_vel_y(-4.0);
            target.set_fall(true);
            // Todo exclusive complex hit animation
            // if target.get_name() == "KFM" {
            //     target.set_state_no(1025)
            // }
            target.set_state(State::A);
            target.add_life(-95);
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1050 => {
            if target_blocking {
                // Original 7
                target.set_vel_x(-4.0);
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            target.set_fall(true);
            target.set_state(State::A);
            // Original 3.5, -7
            target.set_vel_x(-1.0);
            target.set_vel_y(-3.5);
            target.add_life(-80);
            target.set_state_no(state_no + 11);
        }
        1055 => {
            if target_blocking {
                if target_state == &State::A {
                    // Orginal -4
                    target.set_vel_y(-2.0);
                    target.set_vel_y(-4.5);
                } else {
                    // Original 8
                    target.set_vel_x(-4.0);
                }
                target.add_life(-2);
                target.set_state_no(state_no);
                return;
            }
            if target_state == &State::A {
                //Orignal 6
                target.set_vel_x(-3.0);
            } else {
                // Original -4, -5
                target.add_vel_x(-2.0);
                target.add_vel_y(-1.5);
            }
            target.add_life(-35);
            target.set_state_no(state_no + 1);
        }
        1060 => {
            if target_blocking {
                // -7
                target.set_vel_x(-4.0);
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            target.set_fall(true);
            target.set_state(State::A);
            // Original 3.5, -7.5
            target.set_vel_x(-1.0);
            target.set_vel_y(-4.0);
            target.add_life(-90);
            target.set_state_no(state_no + 11);
        }
        1070 => {
            if target_blocking {
                // Original 7
                target.set_vel_x(-4.0);
                target.set_def(true);
                target.add_life(-3);
                target.set_state_no(state_no);
                return;
            }
            // Original -2, -6
            target.add_vel_x(-1.0);
            target.add_vel_y(-2.0);
            target.add_life(-35);
            target.set_state_no(state_no + 11);
        }
        1071 => {
            if target_blocking {
                // Original 7
                target.set_vel_x(-4.0);
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            target.set_fall(true);
            target.set_state(State::A);
            // Original 3.5, -9.5
            target.set_vel_x(-1.0);
            target.set_vel_y(-5.0);
            target.add_life(-68);
            target.set_state_no(state_no + 11);
        }
        1075 => {
            if target_state == &State::A {
                // Original -4, -6
                target.add_vel_x(-1.5);
                target.add_vel_y(-2.0);
            } else {
                //Orignal 6
                target.set_vel_x(-3.0);
            }
            if target_blocking {
                target.set_def(true);
                target.add_life(-2);
                target.set_state_no(state_no);
                return;
            }
            target.add_life(-42);
            target.set_state_no(state_no + 1);
        }
        1100 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -4, -4.5
                    target.add_vel_x(-1.5);
                    target.add_vel_y(-1.5);
                } else {
                    //Orignal 6
                    target.set_vel_x(-3.0);
                }
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            if char.anim_elem == 2 {
                if target_state == &State::A {
                    //Orignal 2
                    target.set_vel_x(-0.5);
                    target.set_vel_y(-2.0);
                } else {
                    // Original -3
                    target.set_vel_x(-1.0);
                }
                target.add_life(-52);
                target.set_state_no(state_no + 11 - low_to_high * 2);
            } else if char.anim_elem == 6 {
                if target_state == &State::A {
                    target.set_vel_x(-1.0);
                    target.set_vel_y(-7.5);
                } else {
                    target.set_vel_x(-1.0);
                    target.set_vel_y(-9.5);
                }
                target.set_fall(true);
                target.add_life(-55);
                target.set_state_no(state_no + 51 - low_to_high * 2);
            }
        }
        1110 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -4, -4.5
                    target.add_vel_x(-1.5);
                    target.add_vel_y(-1.5);
                } else {
                    //Orignal 6
                    target.set_vel_x(-3.0);
                }
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            if char.anim_elem == 2 {
                if target_state == &State::A {
                    //Orignal 2
                    target.set_vel_x(-0.5);
                    target.set_vel_y(-2.0);
                } else {
                    // Original -3
                    target.set_vel_x(-1.0);
                }
                target.add_life(-57);
                target.set_state_no(state_no + 11 - low_to_high * 2);
            } else if char.anim_elem == 6 {
                if target_state == &State::A {
                    target.set_vel_x(-1.0);
                    target.set_vel_y(-8.5);
                } else {
                    // Original 4
                    target.set_vel_x(-1.5);
                    target.set_vel_y(-10.5);
                }
                target.set_fall(true);
                target.add_life(-60);
                target.set_state_no(state_no + 51 - low_to_high * 2);
            }
        }
        1120 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -4, -4.5
                    target.add_vel_x(-1.5);
                    target.add_vel_y(-1.5);
                } else {
                    //Orignal 6
                    target.set_vel_x(-3.0);
                }
                target.set_def(true);
                target.add_life(-4);
                target.set_state_no(state_no);
                return;
            }
            if char.anim_elem == 3 && char.time == 0 {
                if target_state == &State::A {
                    // Orginal 2
                    target.set_vel_x(-0.5);
                    target.set_vel_y(-2.0);
                } else {
                    // Original -3
                    target.set_vel_x(-1.0);
                }
                target.add_life(-30);
                target.set_state_no(state_no + 11 - low_to_high);
            } else if char.anim_elem == 6 {
                if target_state == &State::A {
                    target.set_vel_y(-9.0);
                } else {
                    // Origial -1.2, -11
                    target.set_vel_y(-11.0);
                }
                target.set_vel_x(-1.2);
                target.add_life(-68);
                target.set_state_no(state_no + 51 - low_to_high * 2);
            }
        }
        1200 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -3.5, -4.5
                    target.add_vel_x(-1.2);
                    target.add_vel_y(-1.5);
                } else {
                    //Orignal 7
                    target.set_vel_x(-3.5);
                }
                target.set_def(true);
                target.add_life(-6);
                target.set_state_no(state_no);
                return;
            }
            if target_state == &State::A {
                // Original 3.5
                target.set_vel_x(-2.7);
                target.set_vel_y(-4.5);
            } else {
                // Original 10
                target.set_vel_x(-5.0);
            }
            target.add_life(-100);
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1210 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -4, -4.5
                    target.add_vel_x(-1.5);
                    target.add_vel_y(-1.5);
                } else {
                    //Orignal 8
                    target.set_vel_x(-4.0);
                }
                target.add_life(-9);
                target.set_state_no(state_no);
                return;
            }
            if target_state == &State::A {
                // Original 4
                target.set_vel_x(-3.0);
                target.set_vel_y(-4.5);
            } else {
                // Original 10
                target.set_vel_x(-5.0);
            }
            target.add_life(-125);
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1220 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -5, -5
                    target.add_vel_x(-2.0);
                    target.add_vel_y(-2.0);
                } else {
                    //Orignal 9
                    target.set_vel_x(-4.0);
                }
                target.set_def(true);
                target.add_life(-9);
                target.set_state_no(state_no);
                return;
            }
            if target_state == &State::A {
                // Original 5
                target.set_vel_x(-3.5);
                target.set_vel_y(-5.0);
                target.set_fall(true);
            } else {
                // Original 15
                target.set_vel_x(-7.0);
            }
            target.add_life(-125);
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1400 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -3.5, -4.5
                    target.add_vel_x(-1.0);
                    target.add_vel_y(-1.3);
                } else {
                    //Orignal 9
                    target.set_vel_x(-4.0);
                }
                target.set_def(true);
                target.add_life(-6);
                target.set_state_no(state_no);
                return;
            }
            if target_state == &State::A {
                // Original 2
                target.set_vel_x(-1.0);
                target.set_vel_y(-5.0);
                target.set_fall(true);
            } else {
                // Original 12
                target.set_vel_x(-5.5);
            }
            target.add_life(-100);
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1410 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -3.5, -4.5
                    target.add_vel_x(-1.0);
                    target.add_vel_y(-1.3);
                } else {
                    //Orignal 9
                    target.set_vel_x(-4.0);
                }
                target.set_def(true);
                target.add_life(-6);
                target.set_state_no(state_no);
                return;
            }
            if target_state == &State::A {
                // Original 2
                target.set_vel_x(-1.0);
                target.set_vel_y(-5.0);
                target.set_fall(true);
            } else {
                // Original 12
                target.set_vel_x(-5.5);
            }
            target.add_life(-100);
            target.set_state_no(state_no + 12 - low_to_high);
        }
        1420 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -3.5, -4.5
                    target.add_vel_x(-1.0);
                    target.add_vel_y(-1.3);
                } else {
                    //Orignal 9
                    target.set_vel_x(-4.0);
                }
                target.set_def(true);
                if char.anim_elem == 3 {
                    target.add_life(-8);
                } else {
                    target.add_life(-2);
                }
                target.set_state_no(state_no);
                return;
            }
            if char.anim_elem == 2 {
                if target_state == &State::A {
                    // Original 5
                    target.set_vel_x(-2.0);
                    target.set_vel_y(-4.0);
                } else {
                    // Original 6
                    target.set_vel_x(-3.5);
                }
                target.add_life(-25);
                target.set_state_no(state_no + 12 - low_to_high);
            }
            if char.anim_elem == 3 {
                // Original -5
                target.set_vel_x(-2.0);
                target.set_vel_y(-4.0);
                target.set_state(State::A);
                target.add_life(-100);
                target.set_state_no(state_no + 12 - low_to_high);
                target.set_fall(true);
            }
        }
        3000 => {
            if char.anim_elem == 5 || char.anim_elem == 13 {
                if target_blocking {
                    if target_state == &State::A {
                        // Original -3, -3
                        target.set_vel_x(-1.0);
                        target.add_vel_y(-2.8);
                    } else {
                        //Orignal 6
                        target.set_vel_x(-3.5);
                    }
                    target.set_def(true);
                    target.add_life(-4);
                    target.set_state_no(state_no);
                    return;
                }
                if target_state == &State::A {
                    // Original 3
                    target.set_vel_x(-1.0);
                    target.set_vel_y(-2.8);
                    target.set_fall(true);
                } else {
                    // Original 6
                    target.set_vel_x(-3.5);
                }
                target.add_life(-72);
                target.set_state_no(state_no + 12 - low_to_high);
            } else if char.anim_elem == 21 {
                if target_blocking {
                    if target_state == &State::A {
                        // Original -3, -3
                        target.add_vel_x(-1.0);
                        target.add_vel_y(-1.0);
                    } else {
                        //Orignal 12
                        target.set_vel_x(-5.0);
                    }
                    target.set_def(true);
                    target.add_life(-4);
                    target.set_state_no(state_no);
                    return;
                }
                if target_state == &State::A {
                    // Original 5
                    target.add_vel_x(-3.0);
                    target.add_vel_y(-2.0);
                } else {
                    // Original 3
                    target.add_vel_x(-3.0);
                    target.set_vel_y(-1.0);
                }
                target.set_state(State::A);
                target.set_fall(true);
                target.add_life(-75);
                target.set_state_no(state_no + 12 - low_to_high);
            }
        }
        3050 => {
            if target_blocking {
                if target_state == &State::A {
                    // Original -4.5, -5
                    target.set_vel_x(-2.0);
                    target.set_vel_y(-5.0);
                } else {
                    //Orignal 11
                    target.set_vel_x(-7.5);
                }
                target.set_def(true);
                target.add_life(-12);
                target.set_state_no(state_no);
                return;
            }
            // Original -1.3, -25
            target.set_vel_x(-1.3);
            target.set_vel_y(-25.0);
            target.add_life(-155);
            target.set_state_no(state_no + 51 - low_to_high * 2);
            char.state_no = 3051;
        }
        _ => {}
    }
}

// TODO: scalar product using screen
fn call_state(char: &mut CharData) {
    match char.state_no {
        5 => {
            if char.anim != 5 {
                char.anim = 5;
                char.ctrl = false;
                char.new_anim = true;
                char.current_flip = char.distance > 0.0;
            }
            if char.anim_time == 0 {
                char.state = State::S;
                char.set_vel_x(0.0);
                default_end_action(char);
            }
        }
        6 => {
            if char.anim != 6 {
                char.anim = 6;
                char.ctrl = false;
                char.new_anim = true;
                char.current_flip = char.distance > 0.0;
            }
            if char.anim_time == 0 {
                char.state = State::C;
                char.set_vel_x(0.0);
                default_end_action(char);
            }
        }
        12 => {
            if char.anim != 12 {
                char.anim = 12;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state = State::S;
                default_end_action(char)
            }
        }
        10 => {
            if char.anim != 10 {
                char.anim = 10;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.anim = 11;
                char.set_vel_x(0.0);
                char.state = State::C;
                default_end_action(char)
            }
        }
        40 => {
            if char.anim != 40 {
                char.anim = 40;
                char.state = State::A;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state_no = 50;
                char.vel_y = JUMP_Y;
                if char.direction == 8 {
                    char.set_vel_x(0.0);
                }
                if char.direction == 7 {
                    char.set_vel_x(JUMP_BACK_X);
                }
                if char.direction == 9 {
                    char.set_vel_x(JUMP_FWD_X);
                }
            }
        }
        50 => {
            char.vel_y += 0.45;
            if char.anim != 41 && char.anim != 42 && char.anim != 43 {
                char.new_anim = true;
                if char.vel_x == 0.0 {
                    char.anim = 41;
                }
                if char.vel_x > 0.0 {
                    char.anim = 42;
                }
                if char.vel_x < 0.0 {
                    char.anim = 43;
                }
            }
            if char.action == "x" && char.ctrl {
                char.state_no = 600;
            }
            if char.action == "y" && char.ctrl {
                char.state_no = 610;
            }
            if char.action == "a" && char.ctrl {
                char.state_no = 630;
            }
            if char.action == "b" && char.ctrl {
                char.state_no = 640;
            }
            if char.action == "blocking" && char.ctrl {
                char.state_no = 1340;
            }
            if char.vel_y > 0.0 && char.y >= 500.0 {
                char.state_no = 52;
            }
        }
        52 => {
            if char.anim != 47 {
                char.anim = 47;
                char.y = 500.0;
                char.vel_y = 0.0;
                char.set_vel_x(0.0);
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state = State::S;
                default_end_action(char);
            }
        }
        105 => {
            char.vel_y += 0.45;
            if char.anim != 105 {
                char.state = State::A;
                char.anim = 105;
                char.set_vel_x(RUN_BACK.0);
                char.vel_y = RUNJUMP_BACK.1;
                char.new_anim = true;
            }
            if char.action == "x" && char.ctrl {
                char.state_no = 600;
            }
            if char.action == "y" && char.ctrl {
                char.state_no = 610;
            }
            if char.action == "a" && char.ctrl {
                char.state_no = 630;
            }
            if char.action == "b" && char.ctrl {
                char.state_no = 640;
            }
            if char.action == "blocking" && char.ctrl {
                char.state_no = 1340;
            }
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 106;
            }
        }
        106 => {
            if char.anim != 47 {
                char.state = State::S;
                char.anim = 47;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.time == 0 {
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.y = 500.0;
                default_end_action(char);
            }
        }
        170 => {
            if char.anim != 170 {
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.anim = 170;
                char.new_anim = true;
                char.ctrl = false;
            }
        }
        181 => {
            if char.anim != 181 {
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.anim = 181;
                char.new_anim = true;
                char.ctrl = false;
            }
        }
        640 => {
            char.vel_y += 0.45;
            if char.ctrl && char.action == "a" {
                char.anim = 0;
            }
            if char.anim != 640 {
                char.attack = State::A;
                char.state = State::A;
                char.ctrl = false;
                char.add_power(30);
                char.new_anim = true;
                char.anim = 640;
            }
            if char.vel_y > 0.0 && char.y >= 500.0 {
                char.state_no = 52;
            }
        }
        630 => {
            char.vel_y += 0.45;
            if char.ctrl && char.action == "b" {
                char.anim = 0;
            }
            if char.anim != 630 {
                char.attack = State::A;
                char.state = State::A;
                char.ctrl = false;
                char.add_power(10);
                char.new_anim = true;
                char.anim = 630;
            }
            if char.vel_y > 0.0 && char.y >= 500.0 {
                char.state_no = 52;
            }
        }
        610 => {
            char.vel_y += 0.45;
            if char.ctrl && char.action == "y" {
                char.anim = 0;
            }
            if char.anim != 610 {
                char.attack = State::A;
                char.state = State::A;
                char.ctrl = false;
                char.add_power(30);
                char.new_anim = true;
                char.anim = 610;
            }
            if char.vel_y > 0.0 && char.y >= 500.0 {
                char.state_no = 52;
            }
        }
        600 => {
            char.vel_y += 0.45;
            if char.ctrl && char.action == "x" {
                char.anim = 0;
            }
            if char.anim != 600 {
                char.attack = State::A;
                char.state = State::A;
                char.ctrl = false;
                char.add_power(5);
                char.new_anim = true;
                char.anim = 600;
            }
            if char.anim_elem > 0 {
                char.action = "".to_string();
                char.ctrl = true;
            }
            if char.vel_y > 0.0 && char.y >= 500.0 {
                char.state_no = 52;
            }
        }
        440 => {
            if char.anim != 440 {
                char.attack = State::C;
                char.state = State::C;
                char.anim = 440;
                char.ctrl = false;
                char.add_power(35);
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        410 => {
            if char.anim != 410 {
                char.attack = State::C;
                char.state = State::C;
                char.anim = 410;
                char.ctrl = false;
                char.add_power(25);
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        430 => {
            if char.anim != 430 {
                char.attack = State::C;
                char.state = State::C;
                char.anim = 430;
                char.ctrl = false;
                char.add_power(11);
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        400 => {
            if char.anim != 400 {
                char.attack = State::C;
                char.state = State::C;
                char.ctrl = false;
                char.anim = 400;
                char.add_power(8);
                char.new_anim = true;
            }
            if char.time == 6 {
                char.ctrl = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        240 => {
            if char.anim != 240 {
                char.attack = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.anim = 240;
                char.add_power(30);
                char.new_anim = true;
            }
            if char.anim_elem == 6 {
                // Original 12
                char.add_vel_x(1.0);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        230 => {
            if char.anim != 230 {
                char.attack = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.anim = 230;
                char.add_power(11);
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        210 => {
            if char.anim != 210 {
                char.attack = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.anim = 210;
                char.add_power(30);
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }

        200 => {
            if char.anim != 200 {
                char.attack = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.anim = 200;
                char.add_power(10);
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        195 => {
            if char.anim != 195 {
                char.state = State::S;
                char.ctrl = false;
                char.anim = 195;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        800 => {
            if char.anim != 800 {
                char.anim = 800;
                char.attack = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.new_anim = true;
                char.set_vel_x(0.0);
                char.set_vel_y(0.0);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        810 => {
            if char.anim != 810 {
                char.add_power(40);
                char.anim = 810;
                char.state = State::S;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        820 => {
            if char.anim_elem > 8 {
                char.vel_y += 0.45;
            }
            if char.anim != 820 {
                char.anim = 820;
                char.state = State::A;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.set_vel_y(0.0);
                char.set_y(500.0);
                char.new_anim = true;
            }
            if char.time == 0 {
                if char.anim_elem == 0 {
                    char.add_pos_x(-20.0);
                }
                if char.anim_elem == 1 {
                    // Original 28.0
                    char.add_pos_x(-8.0);
                    char.set_y(500.0);
                }
                if char.anim_elem == 2 {
                    // Original 30.0
                    char.add_pos_x(-2.0);
                }
                if char.anim_elem == 3 {
                    // Original 0.0
                    char.add_pos_x(20.0);
                }
                if char.anim_elem == 4 {
                    // Original -11.0
                    char.add_pos_x(35.0);
                    // Original 0.0
                    char.y += -60.0;
                }
                if char.anim_elem == 5 {
                    // Original -6.0
                    char.add_pos_x(6.0);
                    // Original -60.0
                    char.y += -15.0;
                }
                if char.anim_elem == 6 {
                    // Original -16.0
                    char.add_pos_x(-8.0);
                    // Original -15.0
                    char.y += -15.0;
                }
                if char.anim_elem == 7 {
                    // Original -10.0
                    char.add_pos_x(-20.0);
                    // Original -15.0
                    char.y += -3.0;
                }
                if char.anim_elem == 8 {
                    // Original -20.0
                    char.add_pos_x(-20.0);
                    // Original -6.0
                    char.y += -5.0;
                }
                if char.anim_elem == 9 {
                    char.set_vel_x(-4.0);
                    char.set_vel_y(-4.5)
                }
            }
            if char.time < 0 {
                char.flip_x = true;
                char.state_no = 5050;
            }
        }
        1050 => {
            if char.anim != 1050 {
                char.attack = State::S;
                char.state = State::A;
                char.ctrl = false;
                char.anim = 1050;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
                char.add_power(55);
            }
            if char.anim_elem == 1 {
                // Original 15
                char.add_pos_x(1.5);
            }
            if char.anim_elem == 3 {
                // Origanal 20
                char.add_pos_x(2.0);
            }
            if char.anim_time == 0 {
                char.state_no = 1051
            }
        }
        1051 => {
            char.vel_y += 0.45;
            if char.anim != 1051 {
                char.attack = State::S;
                char.state = State::A;
                char.anim = 1051;
                char.set_vel_x(2.0);
                char.vel_y = -6.0;
                char.new_anim = true;
            }
            if char.vel_y < -1.0 && char.action == "a" || char.action == "b" {
                char.state_no = 1055;
            }
            if char.anim_elem == 3 {
                // Original 20
                char.add_pos_x(2.0);
            }
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 1052
            }
        }

        1020 => {
            if char.anim != 1020 {
                char.state = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.anim = 1020;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
                char.add_power(-330);
            }
            if char.anim_elem == 1 {
                // Original 20
                char.add_pos_x(2.0);
            }
            if char.anim_elem == 2 || char.anim_elem == 11 {
                // Original 10
                char.add_pos_x(1.0);
            }
            if char.anim_elem == 3 {
                // Original 5 & 13
                char.add_pos_x(0.50);
                char.set_vel_x(1.30);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1000 => {
            if char.anim != 1000 {
                char.attack = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.anim = 1000;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
                char.add_power(55);
            }
            if char.anim_elem == 1 {
                // Original 20
                char.add_pos_x(2.0);
            }
            if char.anim_elem == 2 || char.anim_elem == 12 {
                // Original 10
                char.add_pos_x(1.0);
            }
            if char.anim_elem == 4 {
                // Orginal 5
                char.add_pos_x(0.50);
            }
            if char.anim_elem == 8 {
                // Original 5
                char.add_pos_x(-0.5);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1010 => {
            if char.anim != 1010 {
                char.attack = State::S;
                char.state = State::S;
                char.ctrl = false;
                char.anim = 1010;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
                char.add_power(60);
            }
            if char.anim_elem == 1 {
                // Original 20
                char.add_pos_x(2.0);
            }
            if char.anim_elem == 2 || char.anim_elem == 12 {
                // Original 10
                char.add_pos_x(1.0);
            }
            if char.anim_elem == 4 {
                // Original 5 & 4
                char.add_pos_x(0.50);
                char.set_vel_x(0.40);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1052 => {
            if char.anim != 1052 {
                char.anim = 1052;
                char.state = State::S;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
            }
            if char.anim_elem == 0 && char.time == 0 {
                char.y = 500.0;
            }
            if char.anim_elem == 3 {
                // Original -15
                char.add_pos_x(-0.15);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1075 => {
            char.vel_y += 0.45;
            if char.anim != 1055 {
                char.anim = 1055;
                char.state = State::A;
                char.attack = State::S;
                char.new_anim = true;
            }
            if char.time == 0 {
                // Original 10
                char.add_pos_x(0.10);
                char.y -= 0.10;
            }
            if char.vel_y >= -1.0 {
                char.vel_y += 0.2;
            }
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 1052;
            }
        }
        1055 => {
            char.vel_y += 0.45;
            if char.anim != 1055 {
                char.anim = 1055;
                char.attack = State::S;
                char.state = State::A;
                char.new_anim = true;
            }
            if char.time == 0 {
                // Original 10 & -10
                char.add_pos_x(1.0);
                char.y -= 1.0;
            }
            if char.vel_y > 0.0 && char.y >= 490.0 {
                char.state_no = 1056;
            }
        }
        1056 => {
            if char.anim != 1056 {
                char.anim = 1056;
                char.state = State::S;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
            }
            if char.anim_elem == 0 && char.time == 0 {
                char.y = 500.0;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1061 => {
            char.vel_y += 0.45;
            if char.anim != 1061 {
                char.anim = 1061;
                char.state = State::A;
                char.attack = State::S;
                char.set_vel_x(2.5);
                char.vel_y = -7.5;
                char.new_anim = true;
            }
            if (char.action == "a" || char.action == "b") && char.vel_y < -1.0 {
                char.state_no = 1055;
            }
            if char.vel_y > 0.0 && char.y >= 490.0 {
                char.state_no = 1052;
            }
        }
        1060 => {
            if char.anim != 1060 {
                char.ctrl = false;
                char.attack = State::S;
                char.state = State::A;
                char.new_anim = true;
                char.anim = 1060;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.add_power(60);
            }
            if char.anim_elem == 2 {
                // Original 6
                char.add_pos_x(0.6);
            }
            if char.anim_elem == 4 {
                // Original 21
                char.add_pos_x(0.21);
            }
            if char.anim_time == 0 {
                char.state_no = 1061;
            }
        }
        1071 => {
            if char.anim != 1071 {
                char.anim = 1071;
                char.state = State::A;
                char.attack = State::S;
                char.set_vel_x(2.5);
                char.vel_y = -9.0;
                char.new_anim = true;
            }
            if char.vel_y == -8.5 {
                char.time += 20;
            }
            if char.anim_elem == 0 {
                char.vel_y += 0.5;
            }
            if char.vel_y >= -1.0 {
                char.vel_y += 0.2;
            }
            if (char.action == "a" || char.action == "b") && char.vel_y < -1.0 && char.time > 0 {
                char.state_no = 1075;
            }
            if char.vel_y > 0.0 && char.y >= 490.0 {
                char.state_no = 1052;
            }
        }

        1070 => {
            if char.anim != 1070 {
                char.anim = 1070;
                char.ctrl = false;
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.add_power(-330);
            }
            if char.anim_elem == 1 {
                // Original 6
                char.add_pos_x(0.6);
            }
            if char.anim_elem == 3 {
                // original 21
                char.add_pos_x(0.21);
            }
            if char.anim_time == 0 {
                char.state_no = 1071;
            }
        }
        1100 => {
            if char.anim != 1100 {
                char.anim = 1100;
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(55);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1110 => {
            if char.anim != 1110 {
                char.state = State::S;
                char.state = State::S;
                char.new_anim = true;
                char.anim = 1110;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(60);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1120 => {
            if char.anim != 1120 {
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.anim = 1120;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(-330);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1200 => {
            if char.anim != 1200 {
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.anim = 1200;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(50);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1210 => {
            if char.anim != 1210 {
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.anim = 1210;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(60);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1220 => {
            if char.anim != 1220 {
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.anim = 1220;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(-330);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        // Blocking Stuff
        1300 => {
            if char.anim != 1300 {
                char.state = State::S;
                char.new_anim = true;
                char.anim = 1300;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1310 => {
            if char.anim != 1310 {
                char.state = State::S;
                char.new_anim = true;
                char.anim = 1310;
                char.ctrl = false;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1320 => {
            if char.anim != 1320 {
                char.state = State::C;
                char.new_anim = true;
                char.anim = 1320;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1330 => {
            if char.anim != 1330 {
                char.state = State::C;
                char.new_anim = true;
                char.anim = 1330;
                char.ctrl = false;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1340 => {
            char.vel_y += 0.45;
            if char.anim != 1340 {
                char.state = State::A;
                char.new_anim = true;
                char.anim = 1340;
                char.ctrl = false;
            }
            if char.anim_elem == 4 {
                char.ctrl = true;
            }
            if char.y >= 495.0 && char.vel_y > 0.0 {
                char.state_no = 1351;
            }
        }
        1350 => {
            char.vel_y += 0.45;
            if char.anim != 1350 {
                char.state = State::A;
                char.new_anim = true;
                char.anim = 1350;
                char.ctrl = false;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1351 => {
            if char.anim != 47 {
                char.state = State::S;
                char.new_anim = true;
                char.anim = 47;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.y = 500.0;
            }
            if char.time == 3 {
                char.ctrl = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1400 => {
            if char.anim != 1400 {
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.anim = 1400;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(50);
            }
            if char.anim_elem == 1
                || char.anim_elem == 2
                || char.anim_elem == 3
                || char.anim_elem == 7
            {
                // Original 10
                char.add_pos_x(1.0);
            }
            if char.anim_elem == 3 {
                // Orginal 2
                char.set_vel_x(0.2);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1410 => {
            if char.anim != 1410 {
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.anim = 1410;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(50);
            }
            if char.anim_elem == 1
                || char.anim_elem == 2
                || char.anim_elem == 3
                || char.anim_elem == 7
            {
                // Original 10
                char.add_pos_x(1.0);
            }
            if char.anim_elem == 3 {
                // Orginal 8
                char.set_vel_x(0.8);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        1420 => {
            if char.anim != 1420 {
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.anim = 1420;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.ctrl = false;
                char.add_power(-330);
            }
            if char.anim_elem == 1
                || char.anim_elem == 2
                || char.anim_elem == 3
                || char.anim_elem == 7
            {
                // Original 10
                char.add_pos_x(1.0);
            }
            if char.anim_elem == 2 {
                // Original 2
                char.set_vel_x(2.0);
            }
            if char.anim_elem == 3 {
                // Orginal 10
                char.set_vel_x(1.0);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        3000 => {
            if char.anim != 3000 {
                char.anim = 3000;
                char.ctrl = false;
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.add_power(-1000);
            }
            if char.anim_elem == 2
                || char.anim_elem == 10
                || char.anim_elem == 12
                || char.anim_elem == 18
                || char.anim_elem == 20
                || char.anim_elem == 30
            {
                // Original 10
                char.add_pos_x(0.10);
            }
            if char.anim_elem == 1 {
                // Original 20
                char.add_pos_x(0.20);
            }
            if char.anim_elem == 4 {
                // Original 5
                char.add_pos_x(0.50);
            }
            if char.anim_elem == 4 || char.anim_elem == 12 || char.anim_elem == 20 {
                char.set_vel_x(6.0);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        3050 => {
            if char.anim != 3050 {
                char.anim = 3050;
                char.ctrl = false;
                char.state = State::S;
                char.attack = State::S;
                char.new_anim = true;
                char.add_power(-1000);
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        3051 => {
            if char.anim != 3051 {
                char.anim = 3051;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                default_end_action(char);
            }
        }
        5000 => {
            if char.anim != 5000 {
                char.anim = 5000;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                if char.state == State::A {
                    char.state_no = 5035;
                } else {
                    char.state_no = 5005;
                }
            }
        }
        5005 => {
            if char.anim != 5005 {
                char.anim = 5005;
                char.new_anim = true;
            }
            if char.anim_elem == 2 {
                default_end_action(char);
            }
        }
        5001 => {
            if char.anim != 5001 {
                char.anim = 5001;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                if char.state == State::A {
                    char.state_no = 5035;
                } else {
                    char.state_no = 5006;
                }
            }
        }
        5006 => {
            if char.anim != 5006 {
                char.anim = 5006;
                char.new_anim = true;
            }
            if char.anim_elem == 3 {
                default_end_action(char);
            }
        }
        5002 => {
            if char.anim != 5002 {
                char.anim = 5002;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                if char.state == State::A {
                    char.state_no = 5035;
                } else {
                    char.state_no = 5007;
                }
            }
        }
        5007 => {
            if char.anim != 5007 {
                char.anim = 5007;
                char.new_anim = true;
            }
            if char.anim_elem == 3 {
                default_end_action(char);
            }
        }
        5010 => {
            if char.anim != 5010 {
                char.anim = 5010;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                if char.state == State::A {
                    char.state_no = 5035;
                } else {
                    char.state_no = 5015;
                }
            }
        }
        5015 => {
            if char.anim != 5015 {
                char.anim = 5015;
                char.new_anim = true;
            }
            if char.anim_elem == 1 {
                default_end_action(char);
            }
        }
        5011 => {
            if char.anim != 5011 {
                char.anim = 5011;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                if char.state == State::A {
                    char.state_no = 5035;
                } else {
                    char.state_no = 5016;
                }
            }
        }
        5016 => {
            if char.anim != 5016 {
                char.anim = 5016;
                char.new_anim = true;
            }
            if char.anim_elem == 2 {
                default_end_action(char);
            }
        }
        5012 => {
            if char.anim != 5012 {
                char.anim = 5012;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                if char.state == State::A {
                    char.state_no = 5035;
                } else {
                    char.state_no = 5017;
                }
            }
        }
        5017 => {
            if char.anim != 5017 {
                char.anim = 5017;
                char.new_anim = true;
            }
            if char.anim_elem == 1 {
                default_end_action(char);
            }
        }
        5020 => {
            if char.anim != 5020 {
                char.anim = 5020;
                char.state = State::C;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state_no = 5025;
            }
        }
        5025 => {
            if char.anim != 5025 {
                char.anim = 5025;
                char.new_anim = true;
            }
            if char.anim_elem == 1 {
                default_end_action(char);
            }
        }
        5021 => {
            if char.anim != 5021 {
                char.anim = 5021;
                char.state = State::C;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state_no = 5026;
            }
        }
        5026 => {
            if char.anim != 5026 {
                char.anim = 5026;
                char.new_anim = true;
            }
            if char.anim_elem == 2 {
                default_end_action(char);
            }
        }
        5022 => {
            if char.anim != 5022 {
                char.anim = 5022;
                char.state = State::C;
                char.ctrl = false;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state_no = 5027;
            }
        }
        5027 => {
            if char.anim != 5027 {
                char.anim = 5027;
                char.new_anim = true;
            }
            if char.anim_elem == 2 {
                default_end_action(char);
            }
        }
        5030 => {
            char.vel_y += 0.45;
            if char.anim != 5030 {
                char.anim = 5030;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state_no = 5035;
            }
        }
        5035 => {
            char.vel_y += 0.45;
            if char.anim != 5035 {
                char.anim = 5035;
                char.new_anim = true;
            }
            if char.fall {
                if char.anim_time == 0 {
                    char.state_no = 5050;
                }
                if char.vel_y > 0.0 && char.y >= 495.0 {
                    char.state_no = 5100;
                }
            } else {
                if char.anim_time == 0 {
                    char.state_no = 5040;
                }
                if char.vel_y > 0.0 && char.y >= 495.0 {
                    char.state_no = 52;
                }
            }
        }
        5040 => {
            char.vel_y += 0.45;
            if char.anim != 5040 {
                char.anim = 5040;
                char.new_anim = true;
            }
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 52;
            }
        }
        5050 => {
            char.vel_y += 0.45;
            if char.anim != 5050 {
                char.anim = 5050;
                char.new_anim = true;
            }
            // Recovery on future
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 5100;
            }
        }
        5051 => {
            char.vel_y += 0.45;
            if char.anim != 5051 {
                char.state = State::A;
                char.anim = 5051;
                char.new_anim = true;
            }
            if char.vel_y > 0.0 {
                char.state_no = 5061;
            }
        }
        5061 => {
            char.vel_y += 0.45;
            if char.anim != 5061 {
                char.state = State::A;
                char.anim = 5061;
                char.new_anim = true;
            }
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 5101;
            }
        }
        5070 => {
            char.vel_y += 0.45;
            if char.anim != 5070 {
                char.anim = 5070;
                char.state = State::A;
                char.ctrl = false;
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.new_anim = true;
            }
            if char.anim_elem == 1 {
                char.set_vel_x(-2.0);
                char.vel_y = -2.0;
            }
            if char.anim_elem > 0 && char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 5110;
            }
        }
        5100 => {
            if char.anim != 5100 {
                char.set_vel_y(0.0);
                char.y = 500.0;
                char.anim = 5100;
                char.state = State::L;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state_no = 5160
            }
        }
        5101 => {
            char.vel_y += 0.4;
            if char.anim != 5101 {
                char.anim = 5101;
                char.new_anim = true;
            }
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 5160;
            }
        }
        5110 => {
            if char.anim != 5110 {
                char.set_vel_x(0.0);
                char.vel_y = 0.0;
                char.y = 500.0;
                char.anim = 5110;
                char.state = State::L;
                char.new_anim = true;
            }
            if char.anim_time == -30 {
                char.state_no = 5120
            }
        }
        5120 => {
            if char.anim != 5120 {
                char.anim = 5120;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state = State::S;
                default_end_action(char);
            }
        }
        5160 => {
            char.vel_y += 0.4;
            if char.anim != 5160 {
                char.set_y(500.0);
                char.set_vel_y(-2.0);
                char.anim = 5160;
                char.new_anim = true;
            }
            if char.vel_y > 0.0 && char.y >= 495.0 {
                char.state_no = 5170;
            }
        }
        5170 => {
            if char.anim != 5170 {
                char.set_vel_y(0.0);
                char.set_vel_x(0.0);
                char.set_y(500.0);
                char.anim = 5170;
                char.new_anim = true;
            }
            if char.anim_time == 0 {
                char.state_no = 5110;
            }
        }
        _ => {}
    }
}
