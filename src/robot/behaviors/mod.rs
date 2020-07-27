use diesel::PgConnection;

use super::process::*;
use super::robot::Robot;
use crate::grid::*;

impl Robot {
    pub fn respond_to_threats(&mut self, conn: Option<&PgConnection>) -> Option<ProcessResult> {
        // If we just scanned a robot of stronger or unknown capabilites,
        // we want to flee
        let _threats: Vec<&VisibleRobot> = self
            .visible_others
            .iter()
            .filter(|r| r.threat_level != ThreatLevel::Weaker)
            .collect();

        let closest_threat_coords: Option<Coords> = traversal::find_closest_coords(
            self,
            self.visible_others.iter().map(|r| r.coords).collect(),
            false,
        );

        if closest_threat_coords.is_some() {
            let flee_coords = traversal::find_farthest_coords(
                self,
                self.get_known_unoccupied_cells()
                    .keys()
                    .map(|c| c.clone())
                    .collect(),
                true,
                closest_threat_coords,
            );

            if flee_coords.is_some() {
                self.set_status_text(
                    conn,
                    &format!("Must flee from {:?}", closest_threat_coords.unwrap()),
                );
                return Some(ProcessResult::TransitionToFlee(
                    flee_coords.unwrap(),
                    self.data.orientation,
                ));
            }
        }

        None
    }
}
