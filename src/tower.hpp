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
                // Update tower based upon user controls
                void controls();
};

// Store blocks with tower
void storeBlocks(double velocity);
// Scores top goal at a certain speed
void scoreTop(double velocity);
// Scores bottom goal at a certain speed
void scoreBottom(double velocity);
