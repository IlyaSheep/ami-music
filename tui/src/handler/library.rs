use std::sync::Arc;

use tokio::sync::Mutex;

use crate::state::DaemonStates;

pub async fn select_next_track(states: Arc<Mutex<DaemonStates>>) {
    let mut states = states.lock().await;
    if states.library_snapshot.len() > states.library_selected_index + 1 {
        states.library_selected_index = states.library_selected_index.saturating_add(1);
    }
}

pub async fn select_prev_track(states: Arc<Mutex<DaemonStates>>) {
    let mut states = states.lock().await;
    states.library_selected_index = states.library_selected_index.saturating_sub(1);
}
