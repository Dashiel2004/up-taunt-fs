use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*,
        lua2cpp::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
};

unsafe extern "C" fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if status == *FIGHTER_STATUS_KIND_APPEAL {
        WorkModule::on_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE,
        );
        StatusModule::change_status_request_from_script(
            fighter.module_accessor,
            *FIGHTER_STATUS_KIND_FINAL,
            true,
        );
    }
}

pub fn install() {
    Agent::new("fighter")
    .on_line(Main, global_fighter_frame)
    .install();
}
