use serde::Serialize;

use crate::robot::RobotData;

/// Broadcast message of updates to the world
#[derive(Debug, Serialize)]
pub enum BroadcastMessage {
    RobotAttacked,
    RobotMoved { robot: RobotData },
    RobotSpawned,
    RobotExploded,
    ValuableCreated,
    ValuableUpdated,
    ValuableDepleted,
}
