use crate::robot::RobotData;

/// Broadcast message of updates to the world
#[derive(Debug)]
pub enum BroadcastMessage {
    RobotAttacked,
    RobotMoved { robot: RobotData },
    RobotSpawned,
    RobotExploded,
    ValuableCreated,
    ValuableUpdated,
    ValuableDepleted,
}
