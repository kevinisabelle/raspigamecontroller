#ifndef ADVERTISEMENT_H
#define ADVERTISEMENT_H

#include <gio/gio.h>

/* Opaque type for our Advertisement object */
typedef struct _Advertisement Advertisement;

/* Create a new Advertisement.
 *  - connection: the GDBusConnection on which to register the object
 *  - index: an integer used to construct a unique object path
 *  - ad_type: e.g., "peripheral", "broadcast", etc.
 */
Advertisement *advertisement_new(GDBusConnection *connection, int index, const char *ad_type);

/* Set properties on the advertisement object */
void advertisement_set_service_uuids(Advertisement *adv, const char **uuids, int count);
void advertisement_set_appearance(Advertisement *adv, guint16 appearance);
void advertisement_set_local_name(Advertisement *adv, const char *local_name);

/* Register the advertisement on the bus */
void advertisement_register(Advertisement *adv);

/* Retrieve all the advertisement properties as a GVariant dictionary.
 * (This function can be used in a GetAll method implementation.)
 */
GVariant *advertisement_get_all_properties(Advertisement *adv);

/* Free the Advertisement */
void advertisement_free(Advertisement *adv);

#endif // ADVERTISEMENT_H