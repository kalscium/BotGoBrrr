// The tower of the robot

// The state of the tower
class TowerState {
        public:
                // If the little will is toggled down
                bool little_will;
                // If the intake is toggled on
                bool intake;
                // If the parking is toggled on
                bool park;
                TowerState() {
                        little_will = false;
                        intake = false;
                        park = false;
                }
                // Store blocks with tower
                void storeBlocks(double velocity);
                // Spins the toewr at a certain speed
                void spin(double velocity);
                // Update tower based upon user controls
                void controls();
};
