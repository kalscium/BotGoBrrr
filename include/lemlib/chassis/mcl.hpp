#pragma once

#include "lemlib/pose.hpp"
#include "pros/distance.hpp"

namespace lemlib {

// A 'Beam' in the MCL
// Essentially a raycast -- a distance with an angle (from the distance sensor)
class Beam {
        public:
                // IN RADIANS
                float angle;
                // IN INCHES
                float distance;
        Beam(float angle, float distance)
        : angle(angle),
          distance(distance) {}
};

// Particle in the MCL
class Particle {
        public:
                // RADIANS AND INCHES
                lemlib::Pose pose;
                float weight;
        Particle(float x, float y, float theta, float weight)
                : pose(lemlib::Pose(x, y, theta)),
                  weight(weight) {}

        // Updates the pose of a particle with noise
        void updateDeltaNoise(lemlib::Pose deltaPose);
        // Calculates the expected point the beam should land from the current pose
        lemlib::Pose expectedPoint(Beam beam);
        // Distance to the wall -- equivalent to raycasting to the walls and finding the expected beam distance.
        float distanceToWall(lemlib::Pose pose);
        // Updates the weight of a particle from the guassian dist and beams
        void updateWeight(std::vector<Beam> &beams);
};

// Configs for MCL
class MCLConfigs {
        public:
                float N_PARTICLES;
                float GAUSSIAN_STDEV;
                float GAUSSIAN_FACTOR;

                float THETA_NOISE;
                float XY_NOISE;
        MCLConfigs(float N_PARTICLES, float GAUSSIAN_STDEV, float GAUSSIAN_FACTOR, float THETA_NOISE, float XY_NOISE)
                : N_PARTICLES(N_PARTICLES),
                  GAUSSIAN_STDEV(GAUSSIAN_STDEV),
                  GAUSSIAN_FACTOR(GAUSSIAN_FACTOR),
                  THETA_NOISE(THETA_NOISE),
                  XY_NOISE(XY_NOISE) {}
        float gaussian(float x);
};

// A sensor that measures a beam from the robot to the walls (usually a distance sensor)
class Beamer {
        public:
                // The angle of the sensor relative to the yaw of the robot in radians (-pi to pi)
                float angle;
                // The offset from the x in inches
                float x_offset;
                // The offset from the y in inches
                float y_offset;
                // The sensor
                pros::Distance sensor;
        Beamer(float angle, float x_offset, float y_offset, pros::Distance sensor)
        : angle(angle),
          x_offset(x_offset),
          y_offset(y_offset),
          sensor(sensor) {}

        // Calculates the beam accounting for offsets
        Beam getBeam();
};

// Initializes MCL
void initMCL();

/// Runs MCL (used in a cycle) and returns the predicted pose
lemlib::Pose runMCL(std::vector<lemlib::Beam> &beams, lemlib::Pose deltaPose);

} // namespace lemlib
