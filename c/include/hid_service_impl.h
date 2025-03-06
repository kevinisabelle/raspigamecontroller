#ifndef HID_SERVICE_IMPL_H
#define HID_SERVICE_IMPL_H

#include <gio/gio.h>
#include "gamepad_values.h"
#include "bluez/service.h"

/* Opaque type representing the HID GATT Service and its characteristics */
typedef struct _HidGattService HidGattService;

/* Create a new HID GATT Service instance.
 *  - connection: DBus connection to use.
 *  - index: integer used to form the service object path.
 *  - gamepad_values: pointer to a GamepadValues instance used by some characteristics.
 *
 * Returns a pointer to a new HID service instance.
 */
HidGattService *hid_gatt_service_new(GDBusConnection *connection, int index, GamepadValues *gamepad_values);

/* Register the HID service (and all its characteristics) on the D-Bus connection */
void hid_gatt_service_register(HidGattService *hid_service);

/* Free the HID service instance */
void hid_gatt_service_free(HidGattService *hid_service);

#endif // HID_SERVICE_IMPL_H