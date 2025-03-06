#include <stdio.h>
#include <stdlib.h>
#include <gio/gio.h>
#include <glib.h>
#include <hardware.h>         // Provides init_hardware()
#include <gamepad_values.h>   // Provides gamepad_values_new(), gamepad_values_get_report_map(), etc.
#include <agent.h>      // Provides agent_new(), agent_free()
#include <advertisment.h>
#include <bluez_utils.h>      // Provides register_agent(), register_application(), register_advertisement()
#include <hid_service_impl.h> // Provides application_new(), application_get_path(), advertisement_new(), advertisement_get_path(), etc.
#include <updater_service.h>  // Provides gamepad_updater_new(), gamepad_updater_start(), gamepad_updater_stop(), gamepad_updater_free()

int main(int argc, char *argv[])
{
    GError *error = NULL;

    // Initialize hardware
    init_hardware();

    // Create gamepad definition instance
    GamepadValues *gamepadDef = gamepad_values_new();
    if (!gamepadDef)
    {
        fprintf(stderr, "Failed to create GamepadValues instance\n");
        return EXIT_FAILURE;
    }

    printf("Starting GamepadKi...\n");

    // Connect to the system DBus
    GDBusConnection *connection = g_bus_get_sync(G_BUS_TYPE_SYSTEM, NULL, &error);
    if (!connection)
    {
        fprintf(stderr, "Failed to connect to system bus: %s\n", error->message);
        g_error_free(error);
        return EXIT_FAILURE;
    }

    // Create the GLib main loop
    GMainLoop *mainloop = g_main_loop_new(NULL, FALSE);

    // Create and register the agent
    Agent *agent = agent_new(connection, "/com/kevinisabelle/gamepadki/agent");
    register_agent(connection, "/com/kevinisabelle/gamepadki/agent", "KeyboardDisplay");

    // Create the application from the HID service implementation.
    // (Assumes application_new() returns an Application pointer.)
    Application *app = application_new(connection, gamepadDef);
    register_application(connection, application_get_path(app), mainloop);

    // Create and register the advertisement
    // (Assumes advertisement_new() creates an advertisement with a unique object path.)
    Advertisement *advertisement = advertisement_new(connection, 0, "peripheral");
    register_advertisement(connection, advertisement_get_path(advertisement));

    // Print the report map bytes for debugging in hex format
    size_t report_map_length = 0;
    const uint8_t *reportMap = gamepad_values_get_report_map(&report_map_length);
    printf("Report Map Bytes: ");
    for (size_t i = 0; i < report_map_length; i++)
    {
        printf("%02X ", reportMap[i]);
    }
    printf("\n");

    // Create and start the updater service
    GamepadUpdater *updater = gamepad_updater_new(gamepadDef, app, 0.05);
    gamepad_updater_start(updater);

    // Run the main loop
    g_main_loop_run(mainloop);

    // On exit (for example, on SIGINT) perform cleanup:
    gamepad_updater_stop(updater);
    gamepad_updater_free(updater);
    // Free other resources as appropriate:
    agent_free(agent);
    application_free(app);
    advertisement_free(advertisement);
    gamepad_values_free(gamepadDef);
    g_main_loop_unref(mainloop);
    g_object_unref(connection);

    return EXIT_SUCCESS;
}