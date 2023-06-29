use crate::imports::BuildImports::*;

#[acmd_script(//SpecialAirHi
    agent = "pfushigisou", 
    script = "effect_specialairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn game_pfushigisou_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 6.5, 5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5)
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pfushigisou_vine"), Hash40::new("viner5"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pfushigisou_vine"), Hash40::new("viner4"), 0, 0, 0, 0, 0, 0, 1.2, true, 0.7);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pfushigisou_vine"), Hash40::new("viner3"), 0, 0, 0, 0, 0, 0, 1.2, true, 0.5);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pfushigisou_vine_sp_a1"), Hash40::new("tex_pfushigisou_vine_sp_a2"), 4, Hash40::new("viner5"), -1, 0, 0, Hash40::new("viner5"), 6, 0, 0, true, Hash40::new("null"), Hash40::new("viner5"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pfushigisou_vine_sp_b1"), Hash40::new("tex_pfushigisou_vine_sp_b2"), 4, Hash40::new("viner4"), 0, 0, 0, Hash40::new("viner4"), 6, 0, 0, true, Hash40::new("null"), Hash40::new("viner4"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pfushigisou_vine_sp_b1"), Hash40::new("tex_pfushigisou_vine_sp_b2"), 4, Hash40::new("viner3"), 2, 0, 0, Hash40::new("viner3"), 6.5, 0, 0, true, Hash40::new("null"), Hash40::new("viner3"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pfushigisou_vine"), false, true);
        AFTER_IMAGE_OFF(fighter,3);
    }
}    

pub fn install() {
    smashline::install_acmd_scripts!(
        game_pfushigisou_specialairhi
    );
}