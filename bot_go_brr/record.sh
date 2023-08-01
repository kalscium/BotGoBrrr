sudo chmod a+rw /dev/ttyACM0
sudo chmod a+rw /dev/ttyACM1
echo "Recording output..."
nohup prosv5 terminal >> recorded.log 2>&1 &
