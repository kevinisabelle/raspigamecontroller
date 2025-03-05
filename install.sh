sudo cp gamepad.service /etc/systemd/system/

sudo systemctl daemon-reload
sudo systemctl enable gamepad.service
sudo systemctl start gamepad.service