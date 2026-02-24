use {
    smash::{
        lua2cpp::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*},
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

unsafe extern "C" fn up_taunt_final(agent: &mut L2CFighterCommon) {        
    let module_accessor = agent.module_accessor;
    if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FINAL, true);
    }
}