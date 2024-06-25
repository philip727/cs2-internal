pub mod config;
pub mod offsets;
pub mod overlay;
pub mod sdk;
pub mod utils;

use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use config::ConfigContext;
use sdk::{
    entity::data_types::{collision_property::CCollisionProperty, game_scene_node::CGameSceneNode},
    interfaces::{
        engine_client::CEngineClient, game_resource_service::IGameResourceService, CaptureInterface,
    },
};
use utils::module::Module;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HMODULE},
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::GetModuleHandleA,
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        },
    },
};

use crate::sdk::{
    entity::{
        base_entity::{CBaseEntity, CBaseEntitySchema},
        cs_player_controller::CCSPlayerController,
        cs_player_pawn::CCSPlayerPawn,
    },
    interfaces::game_entity_system::WrappedCGameEntitySystem,
};

unsafe fn init() {
    thread::spawn(move || {
        let client = GetModuleHandleA(PCSTR("client.dll\x00".as_ptr()));

        let Ok(client_module) = client else {
            panic!("Unable to client module handle");
        };

        let engine = GetModuleHandleA(PCSTR("engine2.dll\x00".as_ptr()));

        let Ok(engine_module) = engine else {
            panic!("Unable to engine module handle");
        };

        let config_ctx = Arc::new(Mutex::new(ConfigContext::default()));
        let client_module = Module::new(client_module).unwrap();
        let engine_module = Module::new(engine_module).unwrap();
        let Ok(cengine_client) = CEngineClient::capture(&engine_module, "Source2EngineToClient001")
        else {
            panic!("AAA FAILED ENGINE CLIENT");
        };

        let resource_service =
            IGameResourceService::capture(&engine_module, "GameResourceServiceClientV001");
        //let swap_chain_sig =
        //    skidscan::signature!("66 0F 7F 0D ? ? ? ? 66 0F 7F 05 ? ? ? ? 0F 1F 40");
        //rendersystemdx11.dll
        //println!("Hay");
        //let swap_chain = **(std::mem::transmute::<_, *mut *mut *mut ISwapChainDx11>(
        //    memory::resolve_relative_address(
        //        swap_chain_sig.scan_module("rendersystemdx11.dll").unwrap() as *mut c_void,
        //        0x4,
        //        0x8,
        //    ),
        //));

        //println!("Hay 2");
        //let idxgi_swap_chain = (*swap_chain).swap_chain;

        //if idxgi_swap_chain.is_null() {
        //    panic!("NULL SWAP CHAIN AAA");
        //}

        //println!("Hay 3 {idxgi_swap_chain:p}");
        //let device = idxgi_swap_chain.read().GetDevice::<ID3D11Device>();
        //println!("Hay 4");

        //println!("{device:?}");

        ////let idxgi_swap_chain = (**swap_chain).swap_chain;
        //println!("sc 2");

        //println!("Hay 5");

        if let Err(e) = resource_service {
            panic!("{e}");
        }

        let Ok(resource_service) = resource_service else {
            panic!("AAA FAILED GAME RESOURCE SERVICE");
        };

        let entity_system =
            WrappedCGameEntitySystem::init(resource_service.read().game_entity_system);

        overlay::create_overlay(&config_ctx);

        loop {
            std::thread::sleep(Duration::from_millis(100));
            let config_context = { config_ctx.lock().unwrap() };

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

                        let ccs_player_pawn = entity_system.get_entity_by_handle(pawn_handle);

                        let ccs_player_pawn = CCSPlayerPawn(ccs_player_pawn);
                        let health = ccs_player_pawn.get_health();
                        let max_health = ccs_player_pawn.get_max_health();
                        let is_alive = ccs_player_controller.is_alive();

                        let collision =
                            ccs_player_pawn.get_collision_property() as *mut CCollisionProperty;
                        if collision.is_null() {
                            println!("player has no collision property");
                            continue;
                        };

                        let collision = CCollisionProperty(collision);

                        let scene_node = ccs_player_pawn.get_game_scene_node();
                        if scene_node.is_null() {
                            println!("player has no collision property");
                            continue;
                        };

                        let scene_node = CGameSceneNode(scene_node);

                        let transform = scene_node.node_to_world();
                        let pos = transform.vec_position;

                        let vec_mins = collision.get_vec_mins();
                        let vec_maxs = collision.get_vec_maxs();
                        //let Ok(player_name) = ccs_player_controller.sanitized_player_name() else {
                        //    continue;
                        //};

                        if config_context.print_values {
                            println!(
                                "({entity:p}) | alv: {is_alive} | health: {health}/{max_health} | origin: ({pos:?}) "
                            );
                        }
                    }
                }
            }
        }
    });
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
extern "system" fn DllMain(dll_module: HMODULE, reason: u32, lp_reserve: &u32) -> BOOL {
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
