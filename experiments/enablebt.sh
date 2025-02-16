#!/bin/bash
# Restart Bluetooth service
sudo systemctl restart bluetooth

# Bring up the hci0 interface
sudo hciconfig hci0 up

# Make the device discoverable and pairable
sudo hciconfig hci0 piscan

# Use bluetoothctl to set the agent and enable discoverability/pairability
echo -e 'agent NoInputNoOutput\n' | sudo bluetoothctl
echo -e 'default-agent\n' | sudo bluetoothctl
echo -e 'discoverable on\n' | sudo bluetoothctl
echo -e 'pairable on\n' | sudo bluetoothctl
echo -e 'exit\n' | sudo bluetoothctl