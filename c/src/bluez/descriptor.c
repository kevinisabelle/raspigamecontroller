#include <descriptor.h>
#include <gio/gio.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/* Introspection XML for the org.bluez.GattDescriptor1 interface.
 * This minimal XML defines the ReadValue and WriteValue methods.
 */
static const gchar introspection_xml[] =
    "<node>"
    "  <interface name='org.bluez.GattDescriptor1'>"
    "    <method name='ReadValue'>"
    "      <arg type='a{sv}' direction='in' name='options'/>"
    "      <arg type='ay' direction='out' name='value'/>"
    "    </method>"
    "    <method name='WriteValue'>"
    "      <arg type='ay' direction='in' name='value'/>"
    "      <arg type='a{sv}' direction='in' name='options'/>"
    "    </method>"
    "  </interface>"
    "</node>";

/* Private structure definition */
struct _Descriptor
{
    GDBusConnection *connection;
    gchar *object_path;
    gchar *uuid;
    char **flags;
    int flag_count;
    /* The object's parent characteristic path */
    gchar *characteristic_path;
    guint registration_id;
    GDBusNodeInfo *node_info;
};

/* Method call handler for the Descriptor interface.
 * Default implementations return a “Not Supported” error.
 */
static void
descriptor_method_call_handler(GDBusConnection *connection,
                               const gchar *sender,
                               const gchar *object_path,
                               const gchar *interface_name,
                               const gchar *method_name,
                               GVariant *parameters,
                               GDBusMethodInvocation *invocation,
                               gpointer user_data)
{
    Descriptor *desc = (Descriptor *)user_data;
    if (g_strcmp0(method_name, "ReadValue") == 0)
    {
        g_print("Default ReadValue called on %s\n", desc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "ReadValue not implemented");
    }
    else if (g_strcmp0(method_name, "WriteValue") == 0)
    {
        g_print("Default WriteValue called on %s\n", desc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "WriteValue not implemented");
    }
    else
    {
        g_print("Unknown method %s on %s\n", method_name, desc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_INVALID_ARGUMENT,
                                              "Unknown method: %s", method_name);
    }
}

static const GDBusInterfaceVTable descriptor_vtable = {
    descriptor_method_call_handler,
    NULL,
    NULL};

Descriptor *
descriptor_new(GDBusConnection *connection,
               const char *characteristic_path,
               int index,
               const char *uuid,
               const char **flags,
               int flag_count)
{
    Descriptor *desc = malloc(sizeof(Descriptor));
    if (!desc)
        return NULL;
    desc->connection = connection;
    desc->characteristic_path = g_strdup(characteristic_path);
    desc->object_path = g_strdup_printf("%s/desc%d", characteristic_path, index);
    desc->uuid = g_strdup(uuid);
    desc->flag_count = flag_count;
    desc->flags = malloc(sizeof(char *) * flag_count);
    for (int i = 0; i < flag_count; i++)
    {
        desc->flags[i] = g_strdup(flags[i]);
    }
    desc->registration_id = 0;
    desc->node_info = g_dbus_node_info_new_for_xml(introspection_xml, NULL);
    return desc;
}

void descriptor_register(Descriptor *desc)
{
    GError *error = NULL;
    desc->registration_id = g_dbus_connection_register_object(
        desc->connection,
        desc->object_path,
        desc->node_info->interfaces[0],
        &descriptor_vtable,
        desc,
        NULL,
        &error);
    if (desc->registration_id == 0)
    {
        g_printerr("Failed to register descriptor at %s: %s\n",
                   desc->object_path, error->message);
        g_clear_error(&error);
    }
    else
    {
        g_print("Descriptor registered at %s\n", desc->object_path);
    }
}

const char *
descriptor_get_path(Descriptor *desc)
{
    return desc->object_path;
}

/* Build the properties dictionary for org.bluez.GattDescriptor1
 * Properties include:
 *   - Characteristic: parent's object path
 *   - UUID: descriptor UUID
 *   - Flags: array of flag strings
 */
GVariant *
descriptor_get_all_properties(Descriptor *desc)
{
    GVariantBuilder builder;
    g_variant_builder_init(&builder, G_VARIANT_TYPE("a{sv}"));

    /* The "Characteristic" property */
    g_variant_builder_add(&builder, "{sv}",
                          "Characteristic",
                          g_variant_new_string(desc->characteristic_path));
    /* The "UUID" property */
    g_variant_builder_add(&builder, "{sv}",
                          "UUID",
                          g_variant_new_string(desc->uuid));
    /* Add Flags as an array of strings */
    GVariantBuilder flag_builder;
    g_variant_builder_init(&flag_builder, G_VARIANT_TYPE("as"));
    for (int i = 0; i < desc->flag_count; i++)
    {
        g_variant_builder_add(&flag_builder, "s", desc->flags[i]);
    }
    g_variant_builder_add(&builder, "{sv}",
                          "Flags",
                          g_variant_builder_end(&flag_builder));

    return g_variant_builder_end(&builder);
}

void descriptor_free(Descriptor *desc)
{
    if (!desc)
        return;
    if (desc->registration_id > 0)
        g_dbus_connection_unregister_object(desc->connection, desc->registration_id);
    if (desc->object_path)
        g_free(desc->object_path);
    if (desc->uuid)
        g_free(desc->uuid);
    if (desc->characteristic_path)
        g_free(desc->characteristic_path);
    if (desc->flags)
    {
        for (int i = 0; i < desc->flag_count; i++)
        {
            g_free(desc->flags[i]);
        }
        free(desc->flags);
    }
    if (desc->node_info)
        g_dbus_node_info_unref(desc->node_info);
    free(desc);
}