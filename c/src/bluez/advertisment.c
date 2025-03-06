#include <advertisment.h>
#include <gio/gio.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * Introspection XML for org.bluez.LEAdvertisement1.
 * This minimal XML exposes the Release() method.
 */
static const gchar introspection_xml[] =
    "<node>"
    "  <interface name='org.bluez.LEAdvertisement1'>"
    "    <method name='Release'/>"
    "  </interface>"
    "</node>";

/* Private structure definition */
struct _Advertisement
{
    GDBusConnection *connection;
    gchar *object_path;
    gchar *ad_type;
    gchar **service_uuids;
    int service_uuid_count;
    guint16 appearance;
    gchar *local_name;
    guint registration_id;
    GDBusNodeInfo *node_info;
};

static void advertisement_handle_method_call(GDBusConnection *connection,
                                             const gchar *sender,
                                             const gchar *object_path,
                                             const gchar *interface_name,
                                             const gchar *method_name,
                                             GVariant *parameters,
                                             GDBusMethodInvocation *invocation,
                                             gpointer user_data)
{
    Advertisement *adv = (Advertisement *)user_data;
    if (g_strcmp0(method_name, "Release") == 0)
    {
        g_print("Advertisement %s: Released!\n", adv->object_path);
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else
    {
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_INVALID_ARGUMENT,
                                              "Unknown method: %s", method_name);
    }
}

static const GDBusInterfaceVTable adv_vtable = {
    advertisement_handle_method_call,
    NULL,
    NULL};

Advertisement *advertisement_new(GDBusConnection *connection, int index, const char *ad_type)
{
    Advertisement *adv = malloc(sizeof(Advertisement));
    if (!adv)
        return NULL;
    adv->connection = connection;
    adv->registration_id = 0;
    adv->object_path = g_strdup_printf("/org/bluez/gamepadki/advertisement%d", index);
    adv->ad_type = g_strdup(ad_type);
    adv->service_uuids = NULL;
    adv->service_uuid_count = 0;
    adv->appearance = 0;
    adv->local_name = NULL;
    adv->node_info = g_dbus_node_info_new_for_xml(introspection_xml, NULL);
    return adv;
}

void advertisement_set_service_uuids(Advertisement *adv, const char **uuids, int count)
{
    if (!adv)
        return;
    adv->service_uuid_count = count;
    adv->service_uuids = malloc(sizeof(gchar *) * count);
    for (int i = 0; i < count; i++)
    {
        adv->service_uuids[i] = g_strdup(uuids[i]);
    }
}

void advertisement_set_appearance(Advertisement *adv, guint16 appearance)
{
    if (adv)
        adv->appearance = appearance;
}

void advertisement_set_local_name(Advertisement *adv, const char *local_name)
{
    if (adv)
    {
        if (adv->local_name)
            g_free(adv->local_name);
        adv->local_name = g_strdup(local_name);
    }
}

/* Build and return a GVariant dictionary for the advertisement properties.
   This example includes the "Type", "LocalName", and "Appearance" properties.
   Expand as necessary.
*/
GVariant *advertisement_get_all_properties(Advertisement *adv)
{
    GVariantBuilder builder;
    g_variant_builder_init(&builder, G_VARIANT_TYPE("a{sv}"));
    /* Add Type property */
    g_variant_builder_add(&builder, "{sv}", "Type", g_variant_new_string(adv->ad_type));
    if (adv->local_name)
    {
        g_variant_builder_add(&builder, "{sv}", "LocalName", g_variant_new_string(adv->local_name));
    }
    if (adv->appearance > 0)
    {
        g_variant_builder_add(&builder, "{sv}", "Appearance", g_variant_new_uint16(adv->appearance));
    }
    // Additional properties (e.g. ServiceUUIDs, etc.) can be added here.
    return g_variant_builder_end(&builder);
}

void advertisement_register(Advertisement *adv)
{
    GError *error = NULL;
    adv->registration_id = g_dbus_connection_register_object(
        adv->connection,
        adv->object_path,
        adv->node_info->interfaces[0],
        &adv_vtable,
        adv,
        NULL,
        &error);
    if (adv->registration_id == 0)
    {
        g_printerr("Failed to register advertisement at %s: %s\n", adv->object_path, error->message);
        g_clear_error(&error);
    }
    else
    {
        g_print("Advertisement registered at %s\n", adv->object_path);
    }
}

void advertisement_free(Advertisement *adv)
{
    if (!adv)
        return;
    if (adv->registration_id > 0)
        g_dbus_connection_unregister_object(adv->connection, adv->registration_id);
    if (adv->node_info)
        g_dbus_node_info_unref(adv->node_info);
    if (adv->object_path)
        g_free(adv->object_path);
    if (adv->ad_type)
        g_free(adv->ad_type);
    if (adv->local_name)
        g_free(adv->local_name);
    if (adv->service_uuids)
    {
        for (int i = 0; i < adv->service_uuid_count; i++)
        {
            g_free(adv->service_uuids[i]);
        }
        free(adv->service_uuids);
    }
    free(adv);
}