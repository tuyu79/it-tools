use enigo::{Enigo, Keyboard, Settings};
use tauri::Emitter;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
fn paste_at_cursor(text: String) -> Result<(), String> {
    let r = Enigo::new(&Settings::default());
    return match r {
        Ok(mut enigo) => Ok(enigo.text(&text).unwrap()),
        Err(_e) => Err("请授予辅助功能权限".into()),
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // debug
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // register shortcut
            let ctrl_n_shortcut =
                Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyN);
            #[cfg(desktop)]
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |_app, shortcut, event| {
                        println!("{:?}", shortcut);
                        if shortcut == &ctrl_n_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    _app.emit("open-shortcut", ()).unwrap();
                                }
                                ShortcutState::Released => {
                                    println!("Ctrl-N Released!");
                                }
                            }
                        }
                    })
                    .build(),
            )?;

            app.global_shortcut().register(ctrl_n_shortcut)?;

            // config tray
            let main_window = app.get_window("main").unwrap();
            let main_window_clone = main_window.clone(); // need to create a clone

            // 监听窗口关闭事件（新版 API）
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    main_window_clone.hide().unwrap(); // 隐藏而非关闭
                }
            });

            let show_main = MenuItem::with_id(app, "main", "Main", true, None::<&str>)?;
            let show_shortcut = MenuItem::with_id(app, "shortcut", "Shortcut", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_main, &show_shortcut,&quit_i])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app: &AppHandle, event| match event.id.as_ref() {
                    "main" => {
                        app.get_window("main").unwrap().show();
                    }
                    "shortcut" => {
                        let r = app.get_window("shortcut");
                        match r {
                            Some(w) => {
                                w.show().unwrap();
                            }
                            None => {
                                app.emit("open-shortcut", ()).unwrap();
                            }
                        };
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![paste_at_cursor]) // 可以添加其他命令
        .run(tauri::generate_context!())
        .expect("Run Tauri App Failed");
}
