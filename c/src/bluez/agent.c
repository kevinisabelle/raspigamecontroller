#include <agent.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Introspection XML for org.bluez.Agent1 */
static const gchar introspection_xml[] =
    "<node>"
    "  <interface name='org.bluez.Agent1'>"
    "    <method name='Release'/>"
    "    <method name='RequestPasskey'>"
    "      <arg type='o' name='device' direction='in'/>"
    "      <arg type='u' name='passkey' direction='out'/>"
    "    </method>"
    "    <method name='DisplayPasskey'>"
    "      <arg type='o' name='device' direction='in'/>"
    "      <arg type='u' name='passkey' direction='in'/>"
    "    </method>"
    "    <method name='RequestConfirmation'>"
    "      <arg type='o' name='device' direction='in'/>"
    "      <arg type='u' name='passkey' direction='in'/>"
    "    </method>"
    "    <method name='RequestPinCode'>"
    "      <arg type='o' name='device' direction='in'/>"
    "      <arg type='s' name='pincode' direction='out'/>"
    "    </method>"
    "    <method name='RequestAuthorization'>"
    "      <arg type='o' name='device' direction='in'/>"
    "    </method>"
    "    <method name='AuthorizeService'>"
    "      <arg type='o' name='device' direction='in'/>"
    "      <arg type='s' name='uuid' direction='in'/>"
    "    </method>"
    "  </interface>"
    "</node>";

/* Structure definition, private to agent.c */
struct _Agent
{
    GDBusConnection *connection;
    gchar *object_path;
    guint registration_id;
    GDBusNodeInfo *node_info;
};

/* Method call handler for the Agent interface */
static void
handle_method_call(GDBusConnection *connection,
                   const gchar *sender,
                   const gchar *object_path,
                   const gchar *interface_name,
                   const gchar *method_name,
                   GVariant *parameters,
                   GDBusMethodInvocation *invocation,
                   gpointer user_data)
{
    if (g_strcmp0(method_name, "Release") == 0)
    {
        g_print("Agent Released\n");
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else if (g_strcmp0(method_name, "RequestPasskey") == 0)
    {
        const gchar *device;
        g_variant_get(parameters, "(&o)", &device);
        g_print("RequestPasskey for device: %s\n", device);
        g_dbus_method_invocation_return_value(invocation, g_variant_new("(u)", 0));
    }
    else if (g_strcmp0(method_name, "DisplayPasskey") == 0)
    {
        const gchar *device;
        guint passkey;
        g_variant_get(parameters, "(&ou)", &device, &passkey);
        g_print("DisplayPasskey for device: %s, passkey: %u\n", device, passkey);
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else if (g_strcmp0(method_name, "RequestConfirmation") == 0)
    {
        const gchar *device;
        guint passkey;
        g_variant_get(parameters, "(&ou)", &device, &passkey);
        g_print("Auto-confirming for device: %s, passkey: %u\n", device, passkey);
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else if (g_strcmp0(method_name, "RequestPinCode") == 0)
    {
        const gchar *device;
        g_variant_get(parameters, "(&o)", &device);
        g_print("RequestPinCode for device: %s\n", device);
        g_dbus_method_invocation_return_value(invocation, g_variant_new("(s)", "0000"));
    }
    else if (g_strcmp0(method_name, "RequestAuthorization") == 0)
    {
        const gchar *device;
        g_variant_get(parameters, "(&o)", &device);
        g_print("RequestAuthorization for device: %s\n", device);
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else if (g_strcmp0(method_name, "AuthorizeService") == 0)
    {
        const gchar *device;
        const gchar *uuid;
        g_variant_get(parameters, "(&os)", &device, &uuid);
        g_print("AuthorizeService for device: %s with uuid: %s\n", device, uuid);
        g_dbus_method_invocation_return_value(invocation, NULL);
    }
    else
    {
        g_print("Unknown method: %s\n", method_name);
        g_dbus_method_invocation_return_error(invocation,
                                              G_IO_ERROR,
                                              G_IO_ERROR_INVALID_ARGUMENT,
                                              "Unknown method: %s", method_name);
    }
}

/* vtable for the Agent interface */
static const GDBusInterfaceVTable agent_vtable = {
    handle_method_call,
    NULL,
    NULL};

/* Create a new Agent instance */
Agent *agent_new(GDBusConnection *connection, const char *object_path)
{
    Agent *agent = malloc(sizeof(Agent));
    if (!agent)
        return NULL;
    agent->connection = connection;
    agent->object_path = g_strdup(object_path);
    agent->node_info = g_dbus_node_info_new_for_xml(introspection_xml, NULL);
    if (!agent->node_info)
    {
        g_free(agent->object_path);
        free(agent);
        return NULL;
    }
    return agent;
}

/* Register the Agent on the D-Bus connection */
void agent_register(Agent *agent)
{
    GError *error = NULL;
    agent->registration_id = g_dbus_connection_register_object(
        agent->connection,
        agent->object_path,
        agent->node_info->interfaces[0],
        &agent_vtable,
        agent,
        NULL,
        &error);
    if (agent->registration_id == 0)
    {
        g_printerr("Failed to register agent at %s: %s\n", agent->object_path, error->message);
        g_error_free(error);
    }
    else
    {
        g_print("Agent registered at %s\n", agent->object_path);
    }
}

/* Free Agent memory and unregister the object */
void agent_free(Agent *agent)
{
    if (!agent)
        return;
    if (agent->registration_id > 0)
        g_dbus_connection_unregister_object(agent->connection, agent->registration_id);
    if (agent->node_info)
        g_dbus_node_info_unref(agent->node_info);
    g_free(agent->object_path);
    free(agent);
}