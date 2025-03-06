#ifndef SERVICE_H
#define SERVICE_H

#include <gio/gio.h>

typedef struct _Service Service;

/* Create a new Service.
 *  - connection: the DBus connection to use.
 *  - index: an integer used to form a unique object path.
 *  - uuid: the service’s UUID string.
 *  - primary: whether the service is primary (TRUE/FALSE).
 */
Service *service_new(GDBusConnection *connection, int index, const char *uuid, gboolean primary);

/* Register the Service on the D-Bus connection (i.e. export the object). */
void service_register(Service *service);

/* Get the D-Bus object path for this Service. */
const char *service_get_path(Service *service);

/* Add a characteristic’s object path to this Service. */
void service_add_characteristic(Service *service, const char *chrc_path);

/* Build and return a GVariant dictionary containing all properties for
 * the org.bluez.GattService1 interface.
 */
GVariant *service_get_all_properties(Service *service);

/* Free the Service object and unregister it from DBus. */
void service_free(Service *service);

#endif // SERVICE_H