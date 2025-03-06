#ifndef GAMEPAD_CONSTANTS_H
#define GAMEPAD_CONSTANTS_H

// BlueZ and D-Bus Interfaces
#define BLUEZ_AGENT_IFACE "org.bluez.Agent1"
#define BLUEZ_LEADVERTISEMENT_IFACE "org.bluez.LEAdvertisement1"
#define BLUEZ_LEADVERTISEMENT_MANAGER_IFACE "org.bluez.LEAdvertisingManager1"
#define BLUEZ_GATT_CHARACTERISTIC_IFACE "org.bluez.GattCharacteristic1"
#define BLUEZ_GATT_DESCRIPTOR_IFACE "org.bluez.GattDescriptor1"
#define BLUEZ_GATT_SERVICE_IFACE "org.bluez.GattService1"
#define BLUEZ_GATT_MANAGER_IFACE "org.bluez.GattManager1"
#define DBUS_PROPERTIES_IFACE "org.freedesktop.DBus.Properties"
#define DBUS_OM_IFACE "org.freedesktop.DBus.ObjectManager"

// BlueZ service
#define BLUEZ_SERVICE "org.bluez"

// Appearance
#define ADV_APPEARANCE_GAMEPAD 0x03C4

// GATT Service and Characteristic UUIDs
#define GATT_SERVICE_HID_UUID "00001812-0000-1000-8000-00805f9b34fb"
#define GATT_REPORT_MAP_UUID "00002A4B-0000-1000-8000-00805f9b34fb"
#define GATT_REPORT_UUID "00002A4D-0000-1000-8000-00805f9b34fb"
#define GATT_HID_INFORMATION_UUID "00002A4A-0000-1000-8000-00805f9b34fb"
#define GATT_PROTOCOL_MODE_UUID "00002A4E-0000-1000-8000-00805f9b34fb"
#define GATT_HID_CONTROL_POINT_UUID "00002A4C-0000-1000-8000-00805f9b34fb"
#define DEVICE_INFORMATION_SERVICE_UUID "0000180a-0000-1000-8000-00805f9b34fb"
#define BATTERY_SERVICE_UUID "0000180f-0000-1000-8000-00805f9b34fb"

// GATT Descriptor UUIDs
#define GATT_DESC_REPORT_REFERENCE_UUID "00002908-0000-1000-8000-00805f9b34fb"
#define GATT_DESC_CLIENT_DESCRIPTOR_UUID "00002902-0000-1000-8000-00805f9b34fb"

// Other Characteristics
#define SERIAL_NUMBER_CHARACTERISTIC_UUID "00002A25-0000-1000-8000-00805f9b34fb"

// D-Bus Object Paths
#define AGENT_PATH "/com/kevinisabelle/gamepadki/agent"
#define SERVICE_PATH "/org/bluez/gamepadki/service"
#define ADVERTISEMENT_PATH "/org/bluez/gamepadki/advertisement"
#define ADAPTER_PATH "/org/bluez/hci0"
#define BLUEZ_SERVICE_PATH "/org/bluez"

#endif // GAMEPAD_CONSTANTS_H