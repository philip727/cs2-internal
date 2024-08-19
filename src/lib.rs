pub mod config;
pub mod offsets;
pub mod overlay;
pub mod sdk;
pub mod utils;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use config::ConfigContext;
use overlay::esp::ESPContext;
use sdk::{
    data_types::view_matrix::ViewMatrix4x4,
    interfaces::{
        engine_client::{CEngineClient, WrappedCEngineClient},
        game_resource_service::IGameResourceService,
        CaptureInterface,
    },
};
use utils::module::Module;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{BOOL, HMODULE},
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleA},
            SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        },
    },
};

use crate::sdk::interfaces::game_entity_system::WrappedCGameEntitySystem;

unsafe fn init() {
    thread::spawn(move || {
        let client = GetModuleHandleA(PCSTR("client.dll\x00".as_ptr()));

        let Ok(client_module) = client else {
            panic!("Unable to get client module handle");
        };
        let engine = GetModuleHandleA(PCSTR("engine2.dll\x00".as_ptr()));

        let Ok(engine_module) = engine else {
            panic!("Unable to get engine module handle");
        };

        let config_ctx = Arc::new(Mutex::new(ConfigContext::default()));
        let esp_ctx = Arc::new(Mutex::new(ESPContext::default()));

        let client_module = Module::new(client_module).unwrap();
        let engine_module = Module::new(engine_module).unwrap();

        let Ok(cengine_client) = CEngineClient::capture(&engine_module, "Source2EngineToClient001")
        else {
            panic!("AAA FAILED ENGINE CLIENT");
        };

        let resource_service =
            IGameResourceService::capture(&engine_module, "GameResourceServiceClientV001");

        let Ok(cengine_client) = WrappedCEngineClient::init(cengine_client) else {
            panic!("Failed to wrap CEngineClient")
        };

        if let Err(e) = resource_service {
            panic!("{e}");
        }

        let Ok(resource_service) = resource_service else {
            panic!("AAA FAILED GAME RESOURCE SERVICE");
        };

        let entity_system =
            WrappedCGameEntitySystem::init(resource_service.read().game_entity_system);

        // Creates the dx11 hook render loop
        overlay::create_overlay(Arc::clone(&config_ctx), Arc::clone(&esp_ctx));

        // Creates the esp entries for the renderer
        let esp_context_clone = esp_ctx.clone();
        thread::spawn(move || loop {
            std::thread::sleep(Duration::from_millis(1));
            let mut esp_ctx = { esp_context_clone.lock().unwrap() };

            if !cengine_client.in_game() {
                esp_ctx.empty_entries();
                continue;
            }


            let view_matrix = ((client_module.base_addr() + offsets::client_dll::dwViewMatrix)
                as *mut ViewMatrix4x4)
                .read();

            esp_ctx.view_matrix = Some(view_matrix);
            esp_ctx.run_update(&entity_system, &client_module);
        });

        loop {
            std::thread::sleep(Duration::from_millis(10));
        }
    });
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
