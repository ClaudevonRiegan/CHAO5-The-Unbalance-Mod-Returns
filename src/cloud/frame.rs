use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_CLOUD )]
pub fn frame_cloud(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if MotionModule::frame(fighter.module_accessor) > 1.0 {
                ModelModule::set_joint_scale(fighter.module_accessor,Hash40::new("haver"), &Vector3f{x:2.0, y:2.0, z:2.0});
                AttackModule::set_attack_scale(fighter.module_accessor, 1.8, true);
            };
            if MotionModule::frame(fighter.module_accessor) > 26.0 {
                ModelModule::set_joint_scale(fighter.module_accessor,Hash40::new("haver"), &Vector3f{x:1.0, y: 1.0,z: 1.0});         
                AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
            };
        }
        if status_kind == *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2_FALL {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 60.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
            };
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_cloud
    );
}