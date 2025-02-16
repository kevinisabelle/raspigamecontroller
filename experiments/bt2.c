#include <gio/gio.h>
#include <glib.h>
#include <stdio.h>
#include <stdlib.h>

/* Object paths for our application:
   - Our application root: /org/bluez/example
   - HID service: /org/bluez/example/service0
   - Characteristics:
       • Report Map:         /org/bluez/example/service0/char0
       • HID Information:    /org/bluez/example/service0/char1
       • Protocol Mode:      /org/bluez/example/service0/char2
       • Gamepad Report:     /org/bluez/example/service0/char3
*/
#define APP_PATH "/org/bluez/example"
#define SERVICE_PATH APP_PATH "/service0"
#define CHAR_REPORT_MAP_PATH SERVICE_PATH "/char0"
#define CHAR_HID_INFO_PATH SERVICE_PATH "/char1"
#define CHAR_PROTOCOL_MODE_PATH SERVICE_PATH "/char2"
#define CHAR_GAMEPAD_REPORT_PATH SERVICE_PATH "/char3"

/* UUID definitions */
#define HID_SERVICE_UUID "00001812-0000-1000-8000-00805f9b34fb"
#define REPORT_MAP_UUID "00002A4B-0000-1000-8000-00805f9b34fb"
#define HID_INFORMATION_UUID "00002A4A-0000-1000-8000-00805f9b34fb"
#define PROTOCOL_MODE_UUID "00002A4E-0000-1000-8000-00805f9b34fb"
#define REPORT_UUID "00002A4D-0000-1000-8000-00805f9b34fb"

/* Placeholder functions for obtaining gamepad values */
static int get_axis_value(void)
{
    /* Replace this with actual axis reading code */
    return 0;
}

static int get_button_value(void)
{
    /* Replace this with actual button reading code */
    return 0;
}

/* --- D-Bus Introspection Data --- */
/* For simplicity we expose only the GattCharacteristic1 and GattService1 interfaces.
   In a complete implementation you would export a full object hierarchy. */

static const gchar characteristic_introspection_xml[] =
    "<node>"
    "  <interface name='org.bluez.GattCharacteristic1'>"
    "    <method name='ReadValue'>"
    "      <arg type='a{sv}' name='options' direction='in'/>"
    "      <arg type='ay' name='value' direction='out'/>"
    "    </method>"
    "    <method name='WriteValue'>"
    "      <arg type='ay' name='value' direction='in'/>"
    "      <arg type='a{sv}' name='options' direction='in'/>"
    "    </method>"
    "    <method name='StartNotify'/>"
    "    <method name='StopNotify'/>"
    "    <property name='UUID' type='s' access='read'/>"
    "    <property name='Service' type='o' access='read'/>"
    "    <property name='Flags' type='as' access='read'/>"
    "  </interface>"
    "</node>";

static const gchar service_introspection_xml[] =
    "<node>"
    "  <interface name='org.bluez.GattService1'>"
    "    <property name='UUID' type='s' access='read'/>"
    "    <property name='Primary' type='b' access='read'/>"
    "    <property name='Characteristics' type='ao' access='read'/>"
    "  </interface>"
    "</node>";

static GDBusNodeInfo *characteristic_introspection = NULL;
static GDBusNodeInfo *service_introspection = NULL;

/* --- Method Call Handler ---
   This handler is used for all our registered objects.
   We simply check the object path to decide what to return on a ReadValue call. */
static void handle_read_value(GDBusMethodInvocation *invocation, const gchar *object_path)
{
    GVariant *value_variant = NULL;

    if (g_strcmp0(object_path, CHAR_REPORT_MAP_PATH) == 0)
    {
        /* Report Map for a gamepad: one button and one axis.
           This descriptor is similar in concept to a USB HID descriptor. */
        guint8 report_map[] = {
            0x05, 0x01, /* Usage Page (Generic Desktop) */
            0x09, 0x05, /* Usage (Game Pad) */
            0xA1, 0x01, /* Collection (Application) */
            /* Button (1 bit) */
            0x05, 0x09, /*   Usage Page (Button) */
            0x19, 0x01, /*   Usage Minimum (Button 1) */
            0x29, 0x01, /*   Usage Maximum (Button 1) */
            0x15, 0x00, /*   Logical Minimum (0) */
            0x25, 0x01, /*   Logical Maximum (1) */
            0x75, 0x01, /*   Report Size (1) */
            0x95, 0x01, /*   Report Count (1) */
            0x81, 0x02, /*   Input (Data, Variable, Absolute) */
            /* Padding (7 bits) */
            0x75, 0x01, /*   Report Size (1) */
            0x95, 0x07, /*   Report Count (7) */
            0x81, 0x03, /*   Input (Constant) */
            /* Axis (X, 8-bit signed) */
            0x05, 0x01, /*   Usage Page (Generic Desktop) */
            0x09, 0x30, /*   Usage (X axis) */
            0x15, 0x81, /*   Logical Minimum (-127) */
            0x25, 0x7F, /*   Logical Maximum (127) */
            0x75, 0x08, /*   Report Size (8) */
            0x95, 0x01, /*   Report Count (1) */
            0x81, 0x02, /*   Input (Data, Variable, Absolute) */
            0xC0        /* End Collection */
        };
        value_variant = g_variant_new_fixed_array(G_VARIANT_TYPE_BYTE, report_map,
                                                  sizeof(report_map), sizeof(guint8));
    }
    else if (g_strcmp0(object_path, CHAR_HID_INFO_PATH) == 0)
    {
        /* HID Information: version 1.11, country code 0, flags 2 */
        guint8 info[] = {0x11, 0x01, 0x00, 0x02};
        value_variant = g_variant_new_fixed_array(G_VARIANT_TYPE_BYTE, info,
                                                  sizeof(info), sizeof(guint8));
    }
    else if (g_strcmp0(object_path, CHAR_PROTOCOL_MODE_PATH) == 0)
    {
        guint8 mode = 0x01; /* Report Protocol Mode */
        value_variant = g_variant_new_fixed_array(G_VARIANT_TYPE_BYTE, &mode, 1, sizeof(guint8));
    }
    else if (g_strcmp0(object_path, CHAR_GAMEPAD_REPORT_PATH) == 0)
    {
        /* Build the gamepad report: one byte for button state and one byte for axis */
        guint8 button = get_button_value() & 0x01;
        guint8 axis = (guint8)get_axis_value(); /* Assume value is in -127..127 */
        guint8 report[2] = {button, axis};
        value_variant = g_variant_new_fixed_array(G_VARIANT_TYPE_BYTE, report, 2, sizeof(guint8));
    }
    else
    {
        value_variant = g_variant_new("ay", NULL);
    }
    /* Return the value as a tuple (the method signature expects a tuple with one element) */
    g_dbus_method_invocation_return_value(invocation, g_variant_new_tuple(&value_variant, 1));
}

/* --- Global Method Call Dispatcher ---
   For any method call (ReadValue, WriteValue, StartNotify, etc.), we dispatch based on the method name. */
static void on_method_call(GDBusConnection *connection,
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
        handle_read_value(invocation, object_path);
    }
    else if (g_strcmp0(method_name, "WriteValue") == 0)
    {
        /* For simplicity, we ignore writes (except for protocol mode, which you could implement) */
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else if (g_strcmp0(method_name, "StartNotify") == 0 ||
             g_strcmp0(method_name, "StopNotify") == 0)
    {
        /* Notification methods not implemented in this minimal example */
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else
    {
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR, G_IO_ERROR_NOT_SUPPORTED, "Method %s is not supported", method_name);
    }
}

/* --- Property Getter ---
   Returns property values based on the object path and interface. */
static GVariant *on_get_property(GDBusConnection *connection,
                                 const gchar *sender,
                                 const gchar *object_path,
                                 const gchar *interface_name,
                                 const gchar *property_name,
                                 GError **error,
                                 gpointer user_data)
{
    if (g_strcmp0(interface_name, "org.bluez.GattCharacteristic1") == 0)
    {
        if (g_strcmp0(property_name, "UUID") == 0)
        {
            if (g_strcmp0(object_path, CHAR_REPORT_MAP_PATH) == 0)
                return g_variant_new_string(REPORT_MAP_UUID);
            else if (g_strcmp0(object_path, CHAR_HID_INFO_PATH) == 0)
                return g_variant_new_string(HID_INFORMATION_UUID);
            else if (g_strcmp0(object_path, CHAR_PROTOCOL_MODE_PATH) == 0)
                return g_variant_new_string(PROTOCOL_MODE_UUID);
            else if (g_strcmp0(object_path, CHAR_GAMEPAD_REPORT_PATH) == 0)
                return g_variant_new_string(REPORT_UUID);
        }
        else if (g_strcmp0(property_name, "Service") == 0)
        {
            return g_variant_new_object_path(SERVICE_PATH);
        }
        else if (g_strcmp0(property_name, "Flags") == 0)
        {
            if (g_strcmp0(object_path, CHAR_GAMEPAD_REPORT_PATH) == 0)
                return g_variant_new_strv((const gchar *[]){"read", "notify", NULL}, -1);
            else if (g_strcmp0(object_path, CHAR_PROTOCOL_MODE_PATH) == 0)
                return g_variant_new_strv((const gchar *[]){"read", "write", NULL}, -1);
            else
                return g_variant_new_strv((const gchar *[]){"read", NULL}, -1);
        }
    }
    else if (g_strcmp0(interface_name, "org.bluez.GattService1") == 0)
    {
        if (g_strcmp0(property_name, "UUID") == 0)
        {
            return g_variant_new_string(HID_SERVICE_UUID);
        }
        else if (g_strcmp0(property_name, "Primary") == 0)
        {
            return g_variant_new_boolean(TRUE);
        }
        else if (g_strcmp0(property_name, "Characteristics") == 0)
        {
            return g_variant_new_strv((const gchar *[]){
                                          CHAR_REPORT_MAP_PATH,
                                          CHAR_HID_INFO_PATH,
                                          CHAR_PROTOCOL_MODE_PATH,
                                          CHAR_GAMEPAD_REPORT_PATH,
                                          NULL},
                                      -1);
        }
    }
    return NULL;
}

static const GDBusInterfaceVTable interface_vtable = {
    .method_call = on_method_call,
    .get_property = on_get_property,
    .set_property = NULL,
};

/* --- Main ---
   Here we parse the introspection data, connect to the system bus, register our objects,
   and run the main loop. */
int main(int argc, char *argv[])
{
    // print program name
    printf("Program name: %s\n", argv[0]);
    GMainLoop *loop;
    GError *error = NULL;
    guint reg_id;
    GDBusConnection *connection;

    /* Parse introspection data */
    characteristic_introspection = g_dbus_node_info_new_for_xml(characteristic_introspection_xml, &error);
    if (error)
    {
        g_printerr("Error parsing characteristic introspection: %s\n", error->message);
        return 1;
    }
    service_introspection = g_dbus_node_info_new_for_xml(service_introspection_xml, &error);
    if (error)
    {
        g_printerr("Error parsing service introspection: %s\n", error->message);
        return 1;
    }

    loop = g_main_loop_new(NULL, FALSE);

    /* Connect to the system bus */
    connection = g_bus_get_sync(G_BUS_TYPE_SYSTEM, NULL, &error);
    if (error)
    {
        g_printerr("Error connecting to system bus: %s\n", error->message);
        return 1;
    }

    /* Register our GATT service object */
    reg_id = g_dbus_connection_register_object(connection,
                                               SERVICE_PATH,
                                               service_introspection->interfaces[0],
                                               &interface_vtable,
                                               NULL, NULL, &error);
    if (error)
    {
        g_printerr("Error registering service object: %s\n", error->message);
        return 1;
    }

    /* Register our characteristic objects */
    g_dbus_connection_register_object(connection,
                                      CHAR_REPORT_MAP_PATH,
                                      characteristic_introspection->interfaces[0],
                                      &interface_vtable,
                                      NULL, NULL, &error);
    g_dbus_connection_register_object(connection,
                                      CHAR_HID_INFO_PATH,
                                      characteristic_introspection->interfaces[0],
                                      &interface_vtable,
                                      NULL, NULL, &error);
    g_dbus_connection_register_object(connection,
                                      CHAR_PROTOCOL_MODE_PATH,
                                      characteristic_introspection->interfaces[0],
                                      &interface_vtable,
                                      NULL, NULL, &error);
    g_dbus_connection_register_object(connection,
                                      CHAR_GAMEPAD_REPORT_PATH,
                                      characteristic_introspection->interfaces[0],
                                      &interface_vtable,
                                      NULL, NULL, &error);
    if (error)
    {
        g_printerr("Error registering characteristic objects: %s\n", error->message);
        return 1;
    }

    g_print("GATT server running, advertising HID gamepad service...\n");
    g_main_loop_run(loop);

    /* Cleanup */
    g_dbus_node_info_unref(characteristic_introspection);
    g_dbus_node_info_unref(service_introspection);
    g_main_loop_unref(loop);
    return 0;
}
