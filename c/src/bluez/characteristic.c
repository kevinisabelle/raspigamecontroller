#include <characteristic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gio/gio.h>

struct _Characteristic
{
    GDBusConnection *connection;
    gchar *object_path;
    gchar *uuid;
    char **flags;
    int flag_count;
    /* Keep a copy of the parent service path for property "Service" */
    gchar *service_path;
    guint registration_id;
    GDBusNodeInfo *node_info;
};

/* Introspection XML for the org.bluez.GattCharacteristic1 interface.
 * This minimal XML defines methods: ReadValue, WriteValue, StartNotify, StopNotify,
 * and a PropertiesChanged signal. You can extend it as needed.
 */
static const gchar characteristic_introspection_xml[] =
    "<node>"
    "  <interface name='org.bluez.GattCharacteristic1'>"
    "    <method name='ReadValue'>"
    "      <arg direction='in' type='a{sv}' name='options'/>"
    "      <arg direction='out' type='ay' name='value'/>"
    "    </method>"
    "    <method name='WriteValue'>"
    "      <arg direction='in' type='ay' name='value'/>"
    "      <arg direction='in' type='a{sv}' name='options'/>"
    "    </method>"
    "    <method name='StartNotify'/>"
    "    <method name='StopNotify'/>"
    "    <signal name='PropertiesChanged'>"
    "      <arg type='s' name='interface'/>"
    "      <arg type='a{sv}' name='changed'/>"
    "      <arg type='as' name='invalidated'/>"
    "    </signal>"
    "  </interface>"
    "</node>";

/* Method call handler for the Characteristic interface.
 * Currently, default implementations just return a NotSupported error.
 */
static void
characteristic_method_call_handler(GDBusConnection *connection,
                                   const gchar *sender,
                                   const gchar *object_path,
                                   const gchar *interface_name,
                                   const gchar *method_name,
                                   GVariant *parameters,
                                   GDBusMethodInvocation *invocation,
                                   gpointer user_data)
{
    Characteristic *chrc = (Characteristic *)user_data;
    if (g_strcmp0(method_name, "ReadValue") == 0)
    {
        g_print("Default ReadValue called on %s\n", chrc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "ReadValue not implemented");
    }
    else if (g_strcmp0(method_name, "WriteValue") == 0)
    {
        g_print("Default WriteValue called on %s\n", chrc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "WriteValue not implemented");
    }
    else if (g_strcmp0(method_name, "StartNotify") == 0)
    {
        g_print("Default StartNotify called on %s\n", chrc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "StartNotify not implemented");
    }
    else if (g_strcmp0(method_name, "StopNotify") == 0)
    {
        g_print("Default StopNotify called on %s\n", chrc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "StopNotify not implemented");
    }
    else
    {
        g_print("Unknown method %s on %s\n", method_name, chrc->object_path);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_INVALID_ARGUMENT,
                                              "Unknown method: %s", method_name);
    }
}

/* VTable for D-Bus */
static const GDBusInterfaceVTable characteristic_vtable = {
    characteristic_method_call_handler,
    NULL,
    NULL};

/* Create a new Characteristic instance. */
Characteristic *
characteristic_new(GDBusConnection *connection,
                   const char *service_path,
                   int index,
                   const char *uuid,
                   const char **flags,
                   int flag_count)
{
    Characteristic *chrc = malloc(sizeof(Characteristic));
    if (!chrc)
        return NULL;
    chrc->connection = connection;
    chrc->uuid = g_strdup(uuid);
    chrc->flag_count = flag_count;
    chrc->flags = malloc(sizeof(char *) * flag_count);
    for (int i = 0; i < flag_count; i++)
    {
        chrc->flags[i] = g_strdup(flags[i]);
    }
    chrc->service_path = g_strdup(service_path);
    chrc->object_path = g_strdup_printf("%s/char%d", service_path, index);
    chrc->registration_id = 0;
    chrc->node_info = g_dbus_node_info_new_for_xml(characteristic_introspection_xml, NULL);
    return chrc;
}

/* Register the characteristic on DBus */
void characteristic_register(Characteristic *chrc)
{
    GError *error = NULL;
    chrc->registration_id = g_dbus_connection_register_object(
        chrc->connection,
        chrc->object_path,
        chrc->node_info->interfaces[0],
        &characteristic_vtable,
        chrc,
        NULL,
        &error);
    if (chrc->registration_id == 0)
    {
        g_printerr("Failed to register characteristic at %s: %s\n", chrc->object_path, error->message);
        g_clear_error(&error);
    }
    else
    {
        g_print("Characteristic registered at %s\n", chrc->object_path);
    }
}

/* Build the properties dictionary for the GetAll call.
 * It returns a GVariant of type a{sv} containing Service, UUID and Flags.
 */
GVariant *
characteristic_get_all_properties(Characteristic *chrc)
{
    GVariantBuilder builder;
    g_variant_builder_init(&builder, G_VARIANT_TYPE("a{sv}"));
    /* The "Service" property: parent's object path */
    g_variant_builder_add(&builder, "{sv}", "Service", g_variant_new_string(chrc->service_path));
    /* UUID property */
    g_variant_builder_add(&builder, "{sv}", "UUID", g_variant_new_string(chrc->uuid));

    /* Prepare Flags as an array of strings */
    GVariantBuilder flag_builder;
    g_variant_builder_init(&flag_builder, G_VARIANT_TYPE("as"));
    for (int i = 0; i < chrc->flag_count; i++)
    {
        g_variant_builder_add(&flag_builder, "s", chrc->flags[i]);
    }
    g_variant_builder_add(&builder, "{sv}", "Flags", g_variant_builder_end(&flag_builder));

    return g_variant_builder_end(&builder);
}

/* Return the object's path. */
const char *
characteristic_get_path(Characteristic *chrc)
{
    return chrc->object_path;
}

/* Free the Characteristic and unregister it from DBus */
void characteristic_free(Characteristic *chrc)
{
    if (!chrc)
        return;
    if (chrc->registration_id > 0)
        g_dbus_connection_unregister_object(chrc->connection, chrc->registration_id);
    if (chrc->object_path)
        g_free(chrc->object_path);
    if (chrc->uuid)
        g_free(chrc->uuid);
    if (chrc->service_path)
        g_free(chrc->service_path);
    if (chrc->flags)
    {
        for (int i = 0; i < chrc->flag_count; i++)
        {
            g_free(chrc->flags[i]);
        }
        free(chrc->flags);
    }
    if (chrc->node_info)
        g_dbus_node_info_unref(chrc->node_info);
    free(chrc);
}