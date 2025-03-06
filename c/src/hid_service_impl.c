#include "hid_service_impl.h"
#include <service.h>
#include <characteristic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <gio/gio.h>
#include <constants.h>

/* Forward declaration for the default characteristic vtable.
   This is assumed to be defined in your generic characteristic module.
*/
extern const GDBusInterfaceVTable characteristic_vtable;

/* Internal structure for HID service */
struct _HidGattService
{
    Service *service;              /* base service object */
    GDBusConnection *connection;   /* DBus connection reference */
    GamepadValues *gamepad_values; /* Pointer to gamepad values (for ReportMap/Report) */
};

/* ----------------------------------------------------------------------
   CUSTOM METHOD HANDLERS FOR HID-SPECIFIC CHARACTERISTICS
   ---------------------------------------------------------------------- */

/* HID Information Characteristic:
   On ReadValue, returns a fixed 4-byte array: {0x11, 0x01, 0x00, 0x03}
*/
static void hid_info_method_call_handler(GDBusConnection *connection,
                                         const gchar *sender,
                                         const gchar *object_path,
                                         const gchar *interface_name,
                                         const gchar *method_name,
                                         GVariant *parameters,
                                         GDBusMethodInvocation *invocation,
                                         gpointer user_data)
{
    if (g_strcmp0(method_name, "ReadValue") == 0)
    {
        g_print("HID Information read handler called\n");
        guint8 response[] = {0x11, 0x01, 0x00, 0x03};
        GVariant *value = g_variant_new_fixed_array(G_VARIANT_TYPE("y"), response, 4, sizeof(guint8));
        g_dbus_method_invocation_return_value(invocation, g_variant_new("(ay)", value));
    }
    else
    {
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "Method %s not supported", method_name);
    }
}

/* HID Control Point Characteristic:
   On WriteValue, prints the received value.
*/
static void hid_control_point_method_call_handler(GDBusConnection *connection,
                                                  const gchar *sender,
                                                  const gchar *object_path,
                                                  const gchar *interface_name,
                                                  const gchar *method_name,
                                                  GVariant *parameters,
                                                  GDBusMethodInvocation *invocation,
                                                  gpointer user_data)
{
    if (g_strcmp0(method_name, "WriteValue") == 0)
    {
        GVariant *value_variant = NULL;
        g_variant_get(parameters, "(@ay, a{sv})", &value_variant, NULL);
        GVariantIter iter;
        guint8 byte;
        g_print("HID Control Point write handler called\nValue:");
        g_variant_iter_init(&iter, value_variant);
        while (g_variant_iter_loop(&iter, "y", &byte))
        {
            g_print(" %02X", byte);
        }
        g_print("\n");
        g_variant_unref(value_variant);
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else
    {
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_NOT_SUPPORTED,
                                              "Method %s not supported", method_name);
    }
}

/* Vtables for our custom characteristics */
static const GDBusInterfaceVTable hid_info_vtable = {
    hid_info_method_call_handler,
    NULL,
    NULL};

static const GDBusInterfaceVTable hid_control_point_vtable = {
    hid_control_point_method_call_handler,
    NULL,
    NULL};

/* ----------------------------------------------------------------------
   HELPER FUNCTION: REGISTER A CUSTOM CHARACTERISTIC
   ---------------------------------------------------------------------- */
/* This helper registers a characteristic object on DBus using a minimal introspection XML.
   It returns the object path string (which the caller should free) or NULL on failure.
*/
static char *register_custom_characteristic(GDBusConnection *connection,
                                            const char *parent_service_path,
                                            int index,
                                            const char *uuid,
                                            const GDBusInterfaceVTable *vtable,
                                            gpointer user_data)
{
    char *obj_path = g_strdup_printf("%s/char%d", parent_service_path, index);
    /* Minimal introspection XML for a GATT Characteristic */
    const gchar *introspection_xml =
        "<node>"
        "  <interface name='org.bluez.GattCharacteristic1'>"
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
    GDBusNodeInfo *node_info = g_dbus_node_info_new_for_xml(introspection_xml, NULL);
    GError *error = NULL;
    guint registration_id = g_dbus_connection_register_object(
        connection,
        obj_path,
        node_info->interfaces[0],
        vtable,
        user_data,
        NULL,
        &error);
    if (registration_id == 0)
    {
        g_printerr("Failed to register characteristic %s: %s\n", obj_path, error->message);
        g_error_free(error);
        g_dbus_node_info_unref(node_info);
        g_free(obj_path);
        return NULL;
    }
    g_dbus_node_info_unref(node_info);
    return obj_path;
}

/* ----------------------------------------------------------------------
   HID SERVICE IMPLEMENTATION
   ---------------------------------------------------------------------- */
HidGattService *hid_gatt_service_new(GDBusConnection *connection, int index, GamepadValues *gamepad_values)
{
    HidGattService *hid = malloc(sizeof(HidGattService));
    if (!hid)
        return NULL;
    hid->connection = connection;
    hid->gamepad_values = gamepad_values;

    /* Create the base HID service using the generic service module */
    hid->service = service_new(connection, index, GATT_SERVICE_HID_UUID, TRUE);
    if (!hid->service)
    {
        free(hid);
        return NULL;
    }

    /* For demonstration, we add five characteristics.
       Indices 0-2: Dummy characteristics for Report Map, Report, and Protocol Mode.
       We register these using the generic default vtable (assumed to be defined elsewhere).
    */
    const GDBusInterfaceVTable *default_vtable = &characteristic_vtable;
    char *dummy1 = register_custom_characteristic(connection, service_get_path(hid->service), 0, GATT_REPORT_MAP_UUID, default_vtable, NULL);
    if (dummy1)
        service_add_characteristic(hid->service, dummy1);
    char *dummy2 = register_custom_characteristic(connection, service_get_path(hid->service), 1, GATT_REPORT_UUID, default_vtable, NULL);
    if (dummy2)
        service_add_characteristic(hid->service, dummy2);
    char *dummy3 = register_custom_characteristic(connection, service_get_path(hid->service), 2, GATT_PROTOCOL_MODE_UUID, default_vtable, NULL);
    if (dummy3)
        service_add_characteristic(hid->service, dummy3);

    /* Index 3: HID Information Characteristic with custom read */
    char *hid_info_path = register_custom_characteristic(connection, service_get_path(hid->service), 3, GATT_HID_INFORMATION_UUID, &hid_info_vtable, NULL);
    if (hid_info_path)
        service_add_characteristic(hid->service, hid_info_path);

    /* Index 4: HID Control Point Characteristic with custom write */
    char *hid_ctrl_path = register_custom_characteristic(connection, service_get_path(hid->service), 4, GATT_HID_CONTROL_POINT_UUID, &hid_control_point_vtable, NULL);
    if (hid_ctrl_path)
        service_add_characteristic(hid->service, hid_ctrl_path);

    return hid;
}

void hid_gatt_service_register(HidGattService *hid_service)
{
    if (!hid_service)
        return;
    service_register(hid_service->service);
}

void hid_gatt_service_free(HidGattService *hid_service)
{
    if (!hid_service)
        return;
    service_free(hid_service->service);
    free(hid_service);
}