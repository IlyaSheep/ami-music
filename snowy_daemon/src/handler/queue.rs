use snowy_core::library::TrackId;

use crate::states::AppState;

pub async fn enqueue(id: TrackId, state: &mut AppState) {
    if let Some(track) = state.library.tracks.get(&id) {
        state.orchestrator.lock().await.queue.enqueue(track.clone())
    }
}

pub async fn prepend(id: TrackId, state: &mut AppState) {
    if let Some(track) = state.library.tracks.get(&id) {
        state
            .orchestrator
            .lock()
            .await
            .queue
            .prepend_queue(track.clone())
    }
}

pub async fn dequeue(index: usize, state: &mut AppState) {
    state.orchestrator.lock().await.queue.dequeue(index);
}

pub async fn next(state: &mut AppState) {
    state.orchestrator.lock().await.queue.next();
}

pub async fn prev(state: &mut AppState) {
    state.orchestrator.lock().await.queue.prev();
}

pub async fn shuffle(state: &mut AppState) {
    state.orchestrator.lock().await.queue.shuffle();
}

pub async fn clear(state: &mut AppState) {
    state.orchestrator.lock().await.queue.clear();
}
