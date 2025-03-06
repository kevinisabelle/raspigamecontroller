#include <bluez_utils.h>
#include <constants.h>
#include <stdio.h>

/* find_adapter() queries the Object Manager for an object that implements the
 * GATT Manager interface.
 */
char *find_adapter(GDBusConnection *connection)
{
    GError *error = NULL;
    char *adapter_path = NULL;

    /* Call GetManagedObjects on BlueZ */
    GVariant *result = g_dbus_connection_call_sync(connection,
                                                   BLUEZ_SERVICE,
                                                   "/",
                                                   "org.freedesktop.DBus.ObjectManager",
                                                   "GetManagedObjects",
                                                   NULL,
                                                   G_VARIANT_TYPE("(a{oa{sa{sv}}})"),
                                                   G_DBUS_CALL_FLAGS_NONE,
                                                   -1,
                                                   NULL,
                                                   &error);
    if (!result)
    {
        g_printerr("GetManagedObjects error: %s\n", error->message);
        g_error_free(error);
        return NULL;
    }

    /* The result is a tuple with one dictionary argument */
    GVariant *objects;
    g_variant_get(result, "(@a{oa{sa{sv}}})", &objects);

    GVariantIter iter;
    g_variant_iter_init(&iter, objects);
    const gchar *object_path;
    GVariant *interfaces = NULL;
    while (g_variant_iter_loop(&iter, "{&o@a{sa{sv}}}", &object_path, &interfaces))
    {
        if (g_variant_lookup_value(interfaces, BLUEZ_GATT_MANAGER_IFACE, NULL))
        {
            adapter_path = g_strdup(object_path);
            break;
        }
    }
    g_variant_unref(objects);
    g_variant_unref(result);
    return adapter_path;
}

/* register_advertisement() creates a proxy to the LEAdvertisingManager interface
 * on the adapter and calls RegisterAdvertisement.
 */
void register_advertisement(GDBusConnection *connection, const char *adv_path)
{
    GError *error = NULL;
    GDBusProxy *proxy = g_dbus_proxy_new_sync(connection,
                                              G_DBUS_PROXY_FLAGS_NONE,
                                              NULL,
                                              BLUEZ_SERVICE,
                                              ADAPTER_PATH,
                                              BLUEZ_LEADVERTISEMENT_MANAGER_IFACE,
                                              NULL,
                                              &error);
    if (!proxy)
    {
        g_printerr("Failed to create advertisement proxy: %s\n", error->message);
        g_error_free(error);
        return;
    }

    /* Call RegisterAdvertisement(OBJECT_PATH, {}) */
    GVariant *result = g_dbus_proxy_call_sync(proxy,
                                              "RegisterAdvertisement",
                                              g_variant_new("(o, a{sv})", adv_path, NULL),
                                              G_DBUS_CALL_FLAGS_NONE,
                                              -1,
                                              NULL,
                                              &error);
    if (!result)
    {
        g_printerr("Failed to register advertisement: %s\n", error->message);
        g_error_free(error);
    }
    else
    {
        g_print("Advertisement registered at %s\n", adv_path);
        g_variant_unref(result);
    }
    g_object_unref(proxy);
}

/* register_agent() creates a proxy to the AgentManager interface and calls
 * RegisterAgent and RequestDefaultAgent.
 */
void register_agent(GDBusConnection *connection, const char *agent_path, const char *capability)
{
    GError *error = NULL;
    GDBusProxy *proxy = g_dbus_proxy_new_sync(connection,
                                              G_DBUS_PROXY_FLAGS_NONE,
                                              NULL,
                                              BLUEZ_SERVICE,
                                              BLUEZ_SERVICE_PATH,
                                              AGENT_MANAGER_IFACE,
                                              NULL,
                                              &error);
    if (!proxy)
    {
        g_printerr("Failed to create agent manager proxy: %s\n", error->message);
        g_error_free(error);
        return;
    }

    /* Call RegisterAgent(agent_path, capability) */
    GVariant *result = g_dbus_proxy_call_sync(proxy,
                                              "RegisterAgent",
                                              g_variant_new("(os)", agent_path, capability),
                                              G_DBUS_CALL_FLAGS_NONE,
                                              -1,
                                              NULL,
                                              &error);
    if (!result)
    {
        g_printerr("Failed to register agent: %s\n", error->message);
        g_error_free(error);
        g_object_unref(proxy);
        return;
    }
    else
    {
        g_print("Agent registered at %s with capability %s\n", agent_path, capability);
        g_variant_unref(result);
    }

    /* Call RequestDefaultAgent(agent_path) */
    result = g_dbus_proxy_call_sync(proxy,
                                    "RequestDefaultAgent",
                                    g_variant_new("(o)", agent_path),
                                    G_DBUS_CALL_FLAGS_NONE,
                                    -1,
                                    NULL,
                                    &error);
    if (!result)
    {
        g_printerr("Failed to set default agent: %s\n", error->message);
        g_error_free(error);
    }
    else
    {
        g_print("Agent set as default.\n");
        g_variant_unref(result);
    }
    g_object_unref(proxy);

    /* Optionally (as in Python) set adapter properties to Powered, Discoverable, Pairable */
    {
        /* Create a proxy to the adapter Properties interface */
        GDBusProxy *props_proxy = g_dbus_proxy_new_sync(connection,
                                                        G_DBUS_PROXY_FLAGS_NONE,
                                                        NULL,
                                                        BLUEZ_SERVICE,
                                                        ADAPTER_PATH,
                                                        "org.freedesktop.DBus.Properties",
                                                        NULL,
                                                        &error);
        if (!props_proxy)
        {
            g_printerr("Failed to create adapter properties proxy: %s\n", error->message);
            g_error_free(error);
        }
        else
        {
            /* Set Powered = True */
            g_dbus_proxy_call_sync(props_proxy,
                                   "Set",
                                   g_variant_new("(ssv)", "org.bluez.Adapter1", "Powered", g_variant_new_boolean(TRUE)),
                                   G_DBUS_CALL_FLAGS_NONE,
                                   -1,
                                   NULL,
                                   &error);
            /* Set Discoverable = True */
            g_dbus_proxy_call_sync(props_proxy,
                                   "Set",
                                   g_variant_new("(ssv)", "org.bluez.Adapter1", "Discoverable", g_variant_new_boolean(TRUE)),
                                   G_DBUS_CALL_FLAGS_NONE,
                                   -1,
                                   NULL,
                                   &error);
            /* Set Pairable = True */
            g_dbus_proxy_call_sync(props_proxy,
                                   "Set",
                                   g_variant_new("(ssv)", "org.bluez.Adapter1", "Pairable", g_variant_new_boolean(TRUE)),
                                   G_DBUS_CALL_FLAGS_NONE,
                                   -1,
                                   NULL,
                                   &error);
            g_object_unref(props_proxy);
        }
    }
}

/* register_application() finds an adapter, creates a proxy to the GATT Manager,
 * and calls RegisterApplication with app_path.
 */
void register_application(GDBusConnection *connection, const char *app_path, GMainLoop *mainloop)
{
    char *adapter = find_adapter(connection);
    if (!adapter)
    {
        g_printerr("No adapter found\n");
        g_main_loop_quit(mainloop);
        return;
    }

    GError *error = NULL;
    GDBusProxy *proxy = g_dbus_proxy_new_sync(connection,
                                              G_DBUS_PROXY_FLAGS_NONE,
                                              NULL,
                                              BLUEZ_SERVICE,
                                              adapter,
                                              BLUEZ_GATT_MANAGER_IFACE,
                                              NULL,
                                              &error);
    if (!proxy)
    {
        g_printerr("Failed to create GATT manager proxy: %s\n", error->message);
        g_error_free(error);
        g_free(adapter);
        g_main_loop_quit(mainloop);
        return;
    }
    g_print("Registering application at %s...\n", app_path);
    GVariant *result = g_dbus_proxy_call_sync(proxy,
                                              "RegisterApplication",
                                              g_variant_new("(o, a{sv})", app_path, NULL),
                                              G_DBUS_CALL_FLAGS_NONE,
                                              -1,
                                              NULL,
                                              &error);
    if (!result)
    {
        g_printerr("Failed to register application: %s\n", error->message);
        g_error_free(error);
        g_main_loop_quit(mainloop);
    }
    else
    {
        g_print("Application registered successfully.\n");
        g_variant_unref(result);
    }
    g_object_unref(proxy);
    g_free(adapter);
}