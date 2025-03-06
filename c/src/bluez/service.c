#include <service.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <gio/gio.h>

/* Private structure for Service */
struct _Service
{
    GDBusConnection *connection;
    gchar *object_path;
    gchar *uuid;
    gboolean primary;
    /* A pointer array to hold characteristic object paths (strings) */
    GPtrArray *characteristic_paths;
    guint registration_id;
    GDBusNodeInfo *node_info;
};

/* Introspection XML for a minimal org.bluez.GattService1 interface.
   (The GetAll method is provided by the DBus properties interface so
    no methods are explicitly needed here.)
*/
static const gchar introspection_xml[] =
    "<node>"
    "  <interface name='org.bluez.GattService1'>"
    "  </interface>"
    "</node>";

/* For our service object we don’t need to support method calls.
   This handler returns an error for any method call.
*/
static void
service_method_call_handler(GDBusConnection *connection,
                            const gchar *sender,
                            const gchar *object_path,
                            const gchar *interface_name,
                            const gchar *method_name,
                            GVariant *parameters,
                            GDBusMethodInvocation *invocation,
                            gpointer user_data)
{
    g_dbus_method_invocation_return_error(invocation,
                                          G_IO_ERROR,
                                          G_IO_ERROR_NOT_SUPPORTED,
                                          "Method %s is not implemented", method_name);
}

/* vtable for our Service interface */
static const GDBusInterfaceVTable service_vtable = {
    service_method_call_handler,
    NULL,
    NULL};

Service *
service_new(GDBusConnection *connection, int index, const char *uuid, gboolean primary)
{
    Service *service = malloc(sizeof(Service));
    if (!service)
        return NULL;

    service->connection = connection;
    service->uuid = g_strdup(uuid);
    service->primary = primary;
    service->object_path = g_strdup_printf("/org/bluez/gamepadki/service%d", index);
    service->characteristic_paths = g_ptr_array_new_with_free_func(g_free);
    service->registration_id = 0;
    service->node_info = g_dbus_node_info_new_for_xml(introspection_xml, NULL);
    return service;
}

void service_register(Service *service)
{
    GError *error = NULL;
    service->registration_id = g_dbus_connection_register_object(
        service->connection,
        service->object_path,
        service->node_info->interfaces[0],
        &service_vtable,
        service,
        NULL,
        &error);
    if (service->registration_id == 0)
    {
        g_printerr("Failed to register service at %s: %s\n", service->object_path, error->message);
        g_clear_error(&error);
    }
    else
    {
        g_print("Service registered at %s\n", service->object_path);
    }
}

const char *
service_get_path(Service *service)
{
    return service->object_path;
}

void service_add_characteristic(Service *service, const char *chrc_path)
{
    if (service->characteristic_paths == NULL)
        return;
    /* Duplicate the characteristic path string and add it */
    g_ptr_array_add(service->characteristic_paths, g_strdup(chrc_path));
}

GVariant *
service_get_all_properties(Service *service)
{
    GVariantBuilder builder;
    GVariantBuilder chrc_array_builder;

    g_variant_builder_init(&builder, G_VARIANT_TYPE("a{sv}"));
    /* Add UUID property */
    g_variant_builder_add(&builder, "{sv}", "UUID", g_variant_new_string(service->uuid));
    /* Add Primary property */
    g_variant_builder_add(&builder, "{sv}", "Primary", g_variant_new_boolean(service->primary));

    /* Build Characteristics property as an array of object paths (strings) */
    g_variant_builder_init(&chrc_array_builder, G_VARIANT_TYPE("ao"));
    for (guint i = 0; i < service->characteristic_paths->len; i++)
    {
        const char *path = g_ptr_array_index(service->characteristic_paths, i);
        g_variant_builder_add(&chrc_array_builder, "o", path);
    }
    g_variant_builder_add(&builder, "{sv}", "Characteristics", g_variant_builder_end(&chrc_array_builder));

    return g_variant_builder_end(&builder);
}

void service_free(Service *service)
{
    if (!service)
        return;
    if (service->registration_id > 0)
        g_dbus_connection_unregister_object(service->connection, service->registration_id);
    if (service->object_path)
        g_free(service->object_path);
    if (service->uuid)
        g_free(service->uuid);
    if (service->characteristic_paths)
        g_ptr_array_free(service->characteristic_paths, TRUE);
    if (service->node_info)
        g_dbus_node_info_unref(service->node_info);
    free(service);
}