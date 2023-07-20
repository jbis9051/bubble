use crate::public::init::{create_frontend_instance, TokioThread};
use crate::VIRTUAL_MEMORY;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct LocationUpdate {
    pub longitude: f64,
    pub latitude: f64,
    pub timestamp: i64,
    pub altitude: Option<f64>,
    pub floor: Option<i32>,
    pub course: Option<f64>,
    pub horizontal_accuracy: Option<f64>,
    pub vertical_accuracy: Option<f64>,
    pub course_accuracy: Option<f64>,
    pub speed: Option<f64>,
    pub speed_accuracy: Option<f64>,
}

#[derive(Deserialize, Serialize)]
pub struct BackgroundLocationUpdateOptions {
    pub data_directory: String,
    pub update: Vec<LocationUpdate>,
}

pub fn background_location_update(json: &str) {
    // TODO: use an outbox mechanism to send the location updates
    let options: BackgroundLocationUpdateOptions = serde_json::from_str(json).unwrap();
    let instance = {
        let instance = VIRTUAL_MEMORY
            .clone_iter()
            .position(|m| m.static_data.data_directory == options.data_directory);
        if let Some(instance) = instance {
            VIRTUAL_MEMORY.get(instance).unwrap()
        } else {
            let thread = TokioThread::spawn();
            let handle = thread.handle.clone();
            let instance = Arc::new(
                handle
                    .block_on(create_frontend_instance(options.data_directory, thread))
                    .unwrap(),
            );
            VIRTUAL_MEMORY.push(instance.clone());
            instance
        }
    };
    let handle = instance.static_data.tokio.handle.clone();
    handle.block_on(async {
        let groups = instance.get_groups().await.unwrap();
        // TODO: filter for groups that have location updates enabled
        for group in groups {
            for update in &options.update {
                instance
                    .send_location(
                        group.uuid,
                        update.longitude,
                        update.latitude,
                        update.timestamp,
                    )
                    .await
                    .unwrap();
            }
        }
    });
}
