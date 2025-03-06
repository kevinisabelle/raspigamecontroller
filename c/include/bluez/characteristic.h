#ifndef CHARACTERISTIC_H
#define CHARACTERISTIC_H

#include <gio/gio.h>

/* Opaque type for our Characteristic */
typedef struct _Characteristic Characteristic;

/* Create a new Characteristic.
 *  - connection: the DBus connection to use
 *  - service_path: the object path of the parent service (e.g. "/org/bluez/gamepadki/service0")
 *  - index: number used to form a unique characteristic path (e.g. char0, char1, ...)
 *  - uuid: the characteristic’s UUID string
 *  - flags: an array of strings for the characteristic’s flags (e.g. "read", "notify")
 *  - flag_count: number of entries in flags
 */
Characteristic *characteristic_new(GDBusConnection *connection,
                                   const char *service_path,
                                   int index,
                                   const char *uuid,
                                   const char **flags,
                                   int flag_count);

/* Register the characteristic with the D-Bus connection.
 * This will expose the object at its object path using introspection.
 */
void characteristic_register(Characteristic *chrc);

/* Return the object path of the characteristic */
const char *characteristic_get_path(Characteristic *chrc);

/* Get a GVariant dictionary containing all properties for
 * the org.bluez.GattCharacteristic1 interface (to be used in GetAll).
 */
GVariant *characteristic_get_all_properties(Characteristic *chrc);

/* Free the characteristic and unregister it from DBus */
void characteristic_free(Characteristic *chrc);

#endif // CHARACTERISTIC_H