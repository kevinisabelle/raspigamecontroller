#ifndef BLUEZ_UTILS_H
#define BLUEZ_UTILS_H

#include <gio/gio.h>
#include <glib.h>

/* Query BlueZ and return the adapter object path that implements the GATT Manager interface.
 * The returned string must be freed by the caller.
 */
char *find_adapter(GDBusConnection *connection);

/* Register an advertisement.
 * adv_path is the object path of your advertisement object.
 */
void register_advertisement(GDBusConnection *connection, const char *adv_path);

/* Register an agent.
 * agent_path is the object path of your Agent object.
 * capability is a string (e.g. "KeyboardDisplay")
 */
void register_agent(GDBusConnection *connection, const char *agent_path, const char *capability);

/* Register a GATT application.
 * app_path is the root object path for your application.
 * mainloop is the main loop; in case of registration failure the main loop is quit.
 */
void register_application(GDBusConnection *connection, const char *app_path, GMainLoop *mainloop);

#endif // BLUEZ_UTILS_H