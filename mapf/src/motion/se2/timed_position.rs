/*
 * Copyright (C) 2022 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use nalgebra::geometry::Isometry2;
use nalgebra::{Vector2, Vector3};
use time_point::TimePoint;
use crate::motion::{timed, Interpolation, InterpError};


#[derive(Clone, Copy, Debug)]
pub struct Waypoint {
    pub time: TimePoint,
    pub position: Isometry2<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub translational: Vector2<f64>,
    pub rotational: f64,
}

pub struct Motion {
    initial_wp: Waypoint,
    final_wp: Waypoint,
}

impl timed::Timed for Waypoint {
    fn time(&self) -> &TimePoint {
        return &self.time;
    }
    fn set_time(&mut self, new_time: TimePoint) {
        self.time = new_time;
    }
}

impl Waypoint {
    pub fn new(time: TimePoint, x: f64, y: f64, yaw: f64) -> Self {
        return Waypoint{
            time,
            position: Isometry2::new(
                Vector2::new(x, y),
                yaw
            )
        }
    }
}

impl crate::motion::Motion<Isometry2<f64>, Velocity> for Motion {
    fn compute_position(&self, time: &TimePoint) -> Result<Isometry2<f64>, InterpError> {
        if time.nanos_since_zero < self.initial_wp.time.nanos_since_zero {
            return Err(InterpError::OutOfBounds);
        }

        if self.final_wp.time.nanos_since_zero < time.nanos_since_zero {
            return Err(InterpError::OutOfBounds);
        }

        let delta_t = (*time - self.initial_wp.time).as_secs_f64();
        let t_range = (self.final_wp.time - self.initial_wp.time).as_secs_f64();
        return Ok(self.initial_wp.position.lerp_slerp(&self.final_wp.position, delta_t/t_range));
    }

    fn compute_velocity(&self, time: &TimePoint) -> Result<Velocity, InterpError> {
        if time.nanos_since_zero < self.initial_wp.time.nanos_since_zero {
            return Err(InterpError::OutOfBounds);
        }

        if self.final_wp.time.nanos_since_zero < time.nanos_since_zero {
            return Err(InterpError::OutOfBounds);
        }

        // TODO(MXG): Since velocity is taken to be constant across the whole
        // range, this could be precomputed once and saved inside the Motion object.
        let t_range = (self.final_wp.time - self.initial_wp.time).as_secs_f64();
        let p0 = &self.initial_wp.position.translation.vector;
        let p1 = &self.final_wp.position.translation.vector;
        let linear_v = (p1 - p0)/t_range;

        let r0 = &self.initial_wp.position.rotation;
        let r1 = &self.final_wp.position.rotation;
        let angular_v = (r1/r0).angle()/t_range;
        return Ok(
            Velocity{
                translational: linear_v,
                rotational: angular_v,
            }
        );
    }
}

impl Interpolation<Isometry2<f64>, Velocity> for Waypoint {
    type Motion = Motion;

    fn interpolate(&self, up_to: &Self) -> Self::Motion {
        return Self::Motion{
            initial_wp: self.clone(),
            final_wp: up_to.clone()
        }
    }
}

impl crate::motion::trajectory::Waypoint for Waypoint {
    type Position = Isometry2<f64>;
    type Velocity = Velocity;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion::Motion;
    use approx::assert_relative_eq;

    #[test]
    fn test_interpolation_f64() {
        let t0 = time_point::TimePoint::new(0);
        let t1 = t0 + time_point::Duration::from_secs_f64(2.0);
        let wp0 = Waypoint::new(t0, 1.0, 5.0, 10f64.to_radians());
        let wp1 = Waypoint::new(t1, 1.0, 10.0, -20f64.to_radians());

        let motion = wp0.interpolate(&wp1);
        let t = (t1 - t0)/2f64 + t0;
        let p = motion.compute_position(&t).ok().unwrap();
        assert_relative_eq!(p.translation.vector[0], 1f64, max_relative = 0.001);
        assert_relative_eq!(p.translation.vector[1], 7.5f64, max_relative = 0.001);
        assert_relative_eq!(p.rotation.angle(), -5f64.to_radians(), max_relative = 0.001);

        let v = motion.compute_velocity(&t).ok().unwrap();
        assert_relative_eq!(v.translational[0], 0f64, max_relative = 0.001);
        assert_relative_eq!(v.translational[1], 5.0/2.0, max_relative = 0.001);
        assert_relative_eq!(v.rotational, -30f64.to_radians()/2.0, max_relative = 0.001);
    }
}
