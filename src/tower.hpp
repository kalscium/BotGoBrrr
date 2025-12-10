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
                // If the snacky is toggled on
                bool snacky;
                // If we should use color sort
                bool use_color_sort;
                unsigned int time_since_optic;
                TowerState() {
                        little_will = false;
                        intake = false;
                        park = false;
                        snacky = false;
                        time_since_optic = 0;
                        use_color_sort = true;
                }
                // Update tower based upon user controls
                void controls();
                /// Spins the storage motor according to color sort (store opp balls)
                void colorSort(double velocity);

                // Store blocks with tower
                void storeBlocks(double velocity);
                // Scores top goal at a certain speed
                void scoreTop(double velocity);
                // Scores bottom goal at a certain speed
                void scoreBottom(double velocity);
};

/// A test for the optical sensor
void opticTest();
