use crate::state::AppState;

#[tauri::command]
pub fn toggle_timer(window: tauri::Window, state: tauri::State<'_, AppState>) {
  state.timer.toggle();

  window
    .emit("timer-state", state.timer.is_running())
    .unwrap()
}
