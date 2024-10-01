# commands for robotics

# sets the required permissions to allow for uploading of robot code
perms:
    -sudo chmod a+rw /dev/ttyACM0
    -sudo chmod a+rw /dev/ttyACM1

# connects to the v5 brain's terminal
terminal: perms
    prosv5 terminal

# uploads the release (competition) version of the code to the robot
release: perms
    make
    prosv5 ut
