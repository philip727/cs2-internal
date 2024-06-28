
pub mod config;
pub mod offsets;
pub mod overlay;
pub mod sdk;
pub mod utils;

use std::{
    ffi::c_void, ptr::{addr_of_mut, null_mut}, sync::{Arc, Mutex, RwLock}, thread, time::Duration
};

use config::ConfigContext;
use overlay::esp::ESPContext;
use sdk::{
    entity::data_types::{
        collision_property::CCollisionProperty, game_scene_node::CGameSceneNode,
        view_matrix::ViewMatrix4x4,
    },
    interfaces::{
        engine_client::{self, CEngineClient, WrappedCEngineClient},
        game_resource_service::IGameResourceService,
        CaptureInterface,
    },
};
use skidscan::signature;
use utils::module::Module;
use windows::{
    core::{HRESULT, PCSTR},
    Win32::{
        Foundation::{BOOL, HMODULE}, Graphics::Dxgi::IDXGISwapChain, System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleA},
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        }
    },
};
use xorstring::xorstring;

use crate::sdk::{
    entity::{
        base_entity::{CBaseEntity, CBaseEntitySchema},
        cs_player_controller::CCSPlayerController,
        cs_player_pawn::CCSPlayerPawn,
    },
    interfaces::game_entity_system::WrappedCGameEntitySystem,
};

type IDXGIPresentFn = unsafe extern "thiscall" fn(IDXGISwapChain, u32, u32) -> HRESULT;
type CreateHookFn = extern "fastcall" fn(*const c_void, i64, *mut u64, i32) -> u64;

#[repr(transparent)]
struct PresentPtr(IDXGIPresentFn);

unsafe impl Sync for PresentPtr {}
unsafe impl Send for PresentPtr {}

static mut PRESENT_ORIGINAL: Option<IDXGIPresentFn> = None;
static mut COUNTER: i32 = 0;

unsafe extern "fastcall" fn present(this: IDXGISwapChain, sync_interval: u32, flags: u32) -> HRESULT {
    COUNTER += 1;

    return ((PRESENT_ORIGINAL.unwrap()))(this, sync_interval, flags);
}

unsafe fn init() {
    let pattern_hk_sig = signature!("48 8B ?? ?? ?? ?? ?? 48 89 ?? ?? ?? 48 8B ?? ?? ?? ?? ?? 48 89 ?? ?? ?? 89 74");
    let present_addr = pattern_hk_sig.scan_module("gameoverlayrenderer64.dll").unwrap();

    println!("p: {present_addr:p}");

    //let create_hk_sig = signature!("48 89 5C 24 ? 57 48 83 EC ? 33 C0");
    let create_hk_sig = signature!("48 89 5C 24 ? 57 48 83 EC ? 33 C0 ? 89 44 24 50 4D 85 C0");
    let create_hk_addr = create_hk_sig.scan_module("gameoverlayrenderer64.dll").unwrap();
    println!("chk: {create_hk_addr:p}");

    let create_hk = std::mem::transmute::<_, CreateHookFn>(create_hk_addr);
    let my_pressent_addr = present as i64;
    println!("mp: {my_pressent_addr:#0x}");

    let present_original_addr = PRESENT_ORIGINAL;
    println!("op: {present_original_addr:?}");
    println!("counter: {COUNTER}");


    create_hk(present_addr as *const c_void, present as *const i64 as i64, addr_of_mut!(PRESENT_ORIGINAL)as *mut u64, 1);
    let present_original_addr = PRESENT_ORIGINAL;
    println!("op: {present_original_addr:?}");
    println!("counter: {COUNTER}");


    //;thread::spawn(move || {
    //;    let client = GetModuleHandleA(PCSTR("client.dll\x00".as_ptr()));

    //;    let Ok(client_module) = client else {
    //;        panic!("Unable to client module handle");
    //;    };
    //;    let engine = GetModuleHandleA(PCSTR("engine2.dll\x00".as_ptr()));

    //;    let Ok(engine_module) = engine else {
    //;        panic!("Unable to engine module handle");
    //;    };

    //;    let config_ctx = Arc::new(Mutex::new(ConfigContext::default()));
    //;    let esp_ctx = Arc::new(Mutex::new(ESPContext::default()));

    //;    let client_module = Module::new(client_module).unwrap();
    //;    let engine_module = Module::new(engine_module).unwrap();
    //;    let Ok(cengine_client) =
    //;        CEngineClient::capture(&engine_module, "Source2EngineToClient001")
    //;    else {
    //;        panic!("AAA FAILED ENGINE CLIENT");
    //;    };

    //;    let resource_service = IGameResourceService::capture(
    //;        &engine_module,
    //;        "GameResourceServiceClientV001",
    //;    );

    //;    let Ok(cengine_client) = WrappedCEngineClient::init(cengine_client) else {
    //;        panic!("Failed to wrap CEngineClient")
    //;    };

    //;    if let Err(e) = resource_service {
    //;        panic!("{e}");
    //;    }

    //;    let Ok(resource_service) = resource_service else {
    //;        panic!("AAA FAILED GAME RESOURCE SERVICE");
    //;    };

    //;    let entity_system =
    //;        WrappedCGameEntitySystem::init(resource_service.read().game_entity_system);

    //;    // Setup esp
    //;    //let esp_context_clone = esp_ctx.clone();
    //;    //// Creates the dx11 hook render loop
    //;    //overlay::create_overlay(Arc::clone(&config_ctx), Arc::clone(&esp_ctx));

    //;    //let client_mod_clone = client_module.clone();
    //;    //// The esp update loop
    //;    //thread::spawn(move || loop {
    //;    //    std::thread::sleep(Duration::from_millis(1));

    //;    //    let view_matrix = ((client_mod_clone.base_addr() + offsets::client_dll::dwViewMatrix)
    //;    //        as *mut ViewMatrix4x4)
    //;    //        .read();

    //;    //    let mut esp_ctx = { esp_context_clone.lock().unwrap() };
    //;    //    esp_ctx.view_matrix = Some(view_matrix);
    //;    //    esp_ctx.run_update(&entity_system, &cengine_client);
    //;    //});

    //;    //loop {
    //;    //    std::thread::sleep(Duration::from_millis(10));
    //;    //}
    //;    //

    //;    //loop {
    //;    //    std::thread::sleep(Duration::from_millis(100));

    //;    //    let config_context = { config_ctx.lock().unwrap() };
    //;    //    let mut esp_context = { esp_ctx.lock().unwrap() };

    //;    //    if let Ok(in_game) = CEngineClient::get_is_in_game(cengine_client) {
    //;    //        esp_context.entries.clear();

    //;    //        let view_matrix = ((client_module.base_addr() + offsets::client_dll::dwViewMatrix)
    //;    //            as *mut ViewMatrix4x4)
    //;    //            .read();

    //;    //        esp_context.view_matrix = Some(view_matrix);

    //;    //        if in_game {
    //;    //            for i in 1..32 {
    //;    //                let entity = entity_system.get_entity_by_index(i);

    //;    //                if entity.is_null() || !entity.is_aligned() {
    //;    //                    continue;
    //;    //                }

    //;    //                let c_base_entity = CBaseEntity(entity);
    //;    //                let ccs_player_controller: CCSPlayerController = c_base_entity.into();

    //;    //                let pawn_handle = ccs_player_controller.get_pawn_handle();

    //;    //                let ccs_player_pawn = entity_system.get_entity_by_handle(pawn_handle);

    //;    //                let ccs_player_pawn = CCSPlayerPawn(ccs_player_pawn);
    //;    //                let pos = ccs_player_pawn.get_old_origin();

    //;    //                let health = ccs_player_pawn.get_health();
    //;    //                let max_health = ccs_player_pawn.get_max_health();
    //;    //                let is_alive = ccs_player_controller.is_alive();

    //;    //                if is_alive && health > 0 {
    //;    //                    let name = ccs_player_controller.sanitized_player_name();
    //;    //                    let esp_entry = esp_context.create_esp_entry(&pos, name);
    //;    //                    esp_context.entries.push(esp_entry);
    //;    //                }

    //;    //                let collision = ccs_player_pawn.get_collision_property();

    //;    //                if collision.is_null() {
    //;    //                    println!("player has no collision property");
    //;    //                    continue;
    //;    //                };

    //;    //                let collision = CCollisionProperty(collision);

    //;    //                let scene_node = ccs_player_pawn.get_game_scene_node();
    //;    //                if scene_node.is_null() {
    //;    //                    println!("player has no collision property");
    //;    //                    continue;
    //;    //                };

    //;    //                let scene_node = CGameSceneNode(scene_node);

    //;    //                if config_context.print_values {
    //;    //                    println!(
    //;    //                        "(| alv: {is_alive} | health: {health}/{max_health} | origin: ({pos:?}) "
    //;    //                    );
    //;    //                }
    //;    //            }
    //;    //        }
    //;    //    }
    //;    //}
    //;});
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
extern "system" fn DllMain(dll_module: HMODULE, reason: u32, lp_reserve: &u32) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => unsafe {
            let _ = AllocConsole();
            let _ = DisableThreadLibraryCalls(dll_module);
            init();
        },
        DLL_PROCESS_DETACH => unsafe {
            let _ = FreeConsole();
        },
        _ => {}
    };
    BOOL(1)
}
