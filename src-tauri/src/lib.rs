use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use enigo::{Enigo, Keyboard, NewConError, Settings};

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
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![paste_at_cursor]) // 可以添加其他命令
        .run(tauri::generate_context!())
        .expect("Run Tauri App Failed");
}
