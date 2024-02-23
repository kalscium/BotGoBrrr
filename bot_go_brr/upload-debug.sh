#!/bin/bash
# Used like this: `./upload-debug.sh (`simulate` or `record`)`

sudo chmod a+rw /dev/ttyACM0
sudo chmod a+rw /dev/ttyACM1
chmod +x build/upload.sh
cargo run --features $1
/Gata/Programs/pros-cli/pros upload
