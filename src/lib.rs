pub mod gui;
pub mod offsets;
pub mod sdk;
pub mod utils;

use std::{thread, time::Duration};

use gui::GuiContext;
use sdk::interfaces::{
    engine_client::CEngineClient, game_resource_service::IGameResourceService, CaptureInterface,
};
use utils::module::Module;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HANDLE, HWND},
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::GetModuleHandleA,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        },
        UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    },
};

use crate::sdk::{
    entity::{
        base_entity::{CBaseEntity, CBaseEntitySchema},
        cs_player_controller::CCSPlayerController,
        cs_player_pawn::CCSPlayerPawn,
    },
    interfaces::game_entity_system::{CGameEntitySystem, WrappedCGameEntitySystem},
};

unsafe fn init() {
    thread::spawn(move || {
        let client = GetModuleHandleA(PCSTR("client.dll\x00".as_ptr()));
        let engine = GetModuleHandleA(PCSTR("engine2.dll\x00".as_ptr()));

        let Ok(client_module) = client else {
            MessageBoxA(
                HWND(0),
                PCSTR("Failed\x00".as_ptr()),
                PCSTR("Ugh\x00".as_ptr()),
                MB_OK,
            );
            return;
        };

        let Ok(engine_module) = engine else {
            MessageBoxA(
                HWND(0),
                PCSTR("Failed\x00".as_ptr()),
                PCSTR("Ugh\x00".as_ptr()),
                MB_OK,
            );
            return;
        };

        let client_module = Module::new(client_module).unwrap();
        let engine_module = Module::new(engine_module).unwrap();
        let Ok(cengine_client) = CEngineClient::capture(&engine_module, "Source2EngineToClient001")
        else {
            panic!("AAA FAILED ENGINE CLIENT");
        };

        let resource_service =
            IGameResourceService::capture(&engine_module, "GameResourceServiceClientV001");

        if let Err(e) = resource_service {
            panic!("{e}");
        }

        let Ok(resource_service) = resource_service else {
            panic!("AAA FAILED GAME RESOURCE SERVICE");
        };

        println!("Hay");
        GuiContext::initialize();
        println!("Hay 2");

        let entity_system =
            WrappedCGameEntitySystem::init(resource_service.read().game_entity_system);

        loop {
            std::thread::sleep(Duration::from_millis(100));

            if let Ok(in_game) = CEngineClient::get_is_in_game(cengine_client) {
                if in_game {

                    //println!("{highest_index}");
                    for i in 1..64 {
                        let entity = entity_system.get_entity_by_index(i);

                        if entity.is_null() || !entity.is_aligned() {
                            continue;
                        }

                        let c_base_entity = CBaseEntity(entity);
                        let ccs_player_controller: CCSPlayerController = c_base_entity.into();
                        let pawn_handle = ccs_player_controller.get_pawn_handle();

                        let ccs_player_pawn =
                            entity_system.get_entity_by_handle(pawn_handle);

                        let ccs_player_pawn = CCSPlayerPawn(ccs_player_pawn);
                        let health = ccs_player_pawn.get_health();
                        let max_health = ccs_player_pawn.get_max_health();
                        let is_alive = ccs_player_controller.is_alive();

                        //let Ok(player_name) = ccs_player_controller.sanitized_player_name() else {
                        //    continue;
                        //};

                        println!("({entity:p}) | alv: {is_alive} | health: {health}/{max_health}");
                    }
                }
            }
        }
    });
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
extern "system" fn DllMain(dll_module: HANDLE, reason: u32, lp_reserve: &u32) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => unsafe {
            let _ = AllocConsole();
            init();
        },
        DLL_PROCESS_DETACH => unsafe {
            let _ = FreeConsole();
        },
        _ => {}
    };
    BOOL(1)
}
