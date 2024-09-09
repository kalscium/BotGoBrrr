# commands for robotics

# sets the required permissions to allow for uploading of robot code
perms:
    -sudo chmod a+rw /dev/ttyACM0
    -sudo chmod a+rw /dev/ttyACM1
    chmod +x bot_go_brr/build/upload.sh

# connects to the v5 brain's terminal
terminal: perms
    cd bot_go_brr && ~/Programs/pros-cli/pros terminal

# uploads the debug version of the code to the robot and connects to the terminal
debug: perms
    cd bot_go_brr && cargo run

# uploads the release (competition) version of the code to the robot
release: perms
    cd bot_go_brr && cargo run --release

# uploads the record (for autonomous recording) version of the code to the robot
record: perms
    cd bot_go_brr && cargo run --release --features record

# builds and opens the mdbook logbook of BotGoBrrr
book:
    cd logbook && mdbook build && mdbook serve & firefox --new-window http://localhost:3000
