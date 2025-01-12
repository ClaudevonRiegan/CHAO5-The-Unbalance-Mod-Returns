use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
pub fn frame_dolly(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 40.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND {
            fighter.sub_air_check_fall_common();
            if MotionModule::frame(fighter.module_accessor) > 40.0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
                if is_grounded(fighter.module_accessor) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_dolly
    );
}