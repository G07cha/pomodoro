use crate::state::AppState;

#[tauri::command]
pub fn toggle_timer(window: tauri::Window, state: tauri::State<'_, AppState>) {
  let mut timer = state.timer.lock().unwrap();

  timer.toggle();

  window.emit("timer-state", timer.is_running()).unwrap()
}
