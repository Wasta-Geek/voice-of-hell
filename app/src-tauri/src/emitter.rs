use serde::Serialize;
use tauri::{Emitter, Runtime};

pub fn emit_event<R: Runtime, E: Emitter<R>, S: Serialize + Clone> ( app: &E, event: &str, data: &S ) {
    app.emit(event, data).unwrap();
}