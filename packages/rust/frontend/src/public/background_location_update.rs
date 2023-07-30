use crate::public::init::{create_frontend_instance, TokioThread};
use crate::VIRTUAL_MEMORY;
use log::warn;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
pub struct LocationUpdate {
    pub longitude: f64,
    pub latitude: f64,
    pub timestamp: f64,
    pub altitude: Option<f64>,
    pub floor: Option<i32>,
    pub course: Option<f64>,
    pub horizontal_accuracy: Option<f64>,
    pub vertical_accuracy: Option<f64>,
    pub course_accuracy: Option<f64>,
    pub speed: Option<f64>,
    pub speed_accuracy: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BackgroundLocationUpdateOptions {
    pub data_directory: String,
    pub updates: Vec<LocationUpdate>,
}

pub fn background_location_update(options: BackgroundLocationUpdateOptions) -> bool {
    // TODO: use an outbox mechanism to send the location updates
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
        if !instance.logged_in().await {
            return false;
        }
        let groups = instance.get_groups().await.unwrap();
        // TODO: filter for groups that have location updates enabled
        warn!(
            "about to send location {:?} to groups: {:?}",
            options.updates,
            groups.iter().map(|g| g.uuid)
        );
        for group in groups {
            for update in &options.updates {
                instance
                    .send_location(
                        group.uuid,
                        update.longitude,
                        update.latitude,
                        update.timestamp as i64,
                    )
                    .await
                    .map_err(|e| warn!("error sending location: {:?}", e))
                    .unwrap();
            }
        }
        true
    })
}
