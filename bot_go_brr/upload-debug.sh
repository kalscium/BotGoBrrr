sudo chmod a+rw /dev/ttyACM0
sudo chmod a+rw /dev/ttyACM1
chmod +x build/upload.sh
cargo run
/Gata/Programs/pros-cli/pros upload