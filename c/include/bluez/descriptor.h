#ifndef DESCRIPTOR_H
#define DESCRIPTOR_H

#include <gio/gio.h>

/* Opaque type for our Descriptor */
typedef struct _Descriptor Descriptor;

/* Create a new Descriptor.
 *  - connection: the DBus connection to use
 *  - characteristic_path: the parent characteristic’s object path
 *  - index: an integer used to form a unique descriptor path (e.g. desc0, desc1)
 *  - uuid: the descriptor’s UUID string
 *  - flags: an array of strings (e.g. "read", "write")
 *  - flag_count: number of flag strings
 */
Descriptor *descriptor_new(GDBusConnection *connection,
                           const char *characteristic_path,
                           int index,
                           const char *uuid,
                           const char **flags,
                           int flag_count);

/* Register the descriptor with the DBus connection */
void descriptor_register(Descriptor *desc);

/* Return the descriptor object path */
const char *descriptor_get_path(Descriptor *desc);

/* Build and return a GVariant dictionary containing all properties
 * for the org.bluez.GattDescriptor1 interface.
 */
GVariant *descriptor_get_all_properties(Descriptor *desc);

/* Free the Descriptor and unregister from DBus */
void descriptor_free(Descriptor *desc);

#endif // DESCRIPTOR_H