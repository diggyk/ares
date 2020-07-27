use diesel::PgConnection;

use super::process::*;
use super::robot::Robot;
use crate::grid::*;

impl Robot {
    /// Check to see if we scanned some threats.  If so, transition to flee
    fn check_for_threats(&mut self, conn: Option<&PgConnection>) -> Option<ProcessResult> {
        let _threats: Vec<&VisibleRobot> = self
            .visible_others
            .iter()
            .filter(|r| {
                r.threat_level == ThreatLevel::Stronger || r.threat_level == ThreatLevel::Unknown
            })
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

    /// Check to see if there are targets we should pursue
    fn check_for_targets(&mut self, conn: Option<&PgConnection>) -> Option<ProcessResult> {
        let _targets: Vec<&VisibleRobot> = self
            .visible_others
            .iter()
            .filter(|r| {
                r.threat_level == ThreatLevel::Weaker || r.threat_level == ThreatLevel::Equal
            })
            .collect();

        let closest_target_coords: Option<Coords> = traversal::find_closest_coords(
            self,
            self.visible_others.iter().map(|r| r.coords).collect(),
            false,
        );

        if closest_target_coords.is_some() {
            let pursuit_coords = traversal::find_closest_coords(
                self,
                self.get_known_unoccupied_cells()
                    .keys()
                    .map(|c| c.clone())
                    .collect(),
                true,
            );

            if pursuit_coords.is_some() {
                let path_to_coords = traversal::find_path(self, pursuit_coords.unwrap());
                if path_to_coords.is_err() {
                    return None;
                }

                let mut target_id: Option<i64> = None;
                for target in &self.visible_others {
                    if target.coords == pursuit_coords.unwrap() {
                        target_id = Some(target.robot_id);
                    }
                }

                self.set_status_text(
                    conn,
                    &format!("I'm going to attach Robot {}", target_id.unwrap()),
                );
                return Some(ProcessResult::TransitionToPursuit(target_id.unwrap()));
            }
        }

        None
    }

    /// Respond to scanned robots by fleeing, attacking, or ignoring
    pub fn respond_to_others(&mut self, conn: Option<&PgConnection>) -> Option<ProcessResult> {
        // If we just scanned a robot of stronger or unknown capabilites,
        // we want to flee

        if let Some(response) = self.check_for_threats(conn) {
            return Some(response);
        }

        if let Some(response) = self.check_for_targets(conn) {
            return Some(response);
        }

        None
    }
}
