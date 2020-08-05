use serde::Serialize;

use crate::grid::GridCell;
use crate::robot::{RobotData, RobotInfo};
use crate::valuable::Valuable;

/// Broadcast message of updates to the world
#[derive(Debug, Serialize)]
pub enum BroadcastMessage {
    InitializerData {
        id: usize,
        cells: Vec<GridCell>,
        robots: Vec<RobotInfo>,
        valuables: Vec<Valuable>,
    },
    RobotAttacked {
        attacker_id: i64,
        target_id: i64,
    },
    RobotMoved {
        robot: RobotData,
    },
    RobotSpawned {
        robot: RobotInfo,
    },
    RobotExploded {
        robot_id: i64,
    },
    ValuableCreated {
        valuable: Valuable,
    },
    ValuableUpdated {
        valuable: Valuable,
    },
    ValuableDepleted {
        valuable_id: i64,
    },
}
