#include <gio/gio.h>
#include <string>

class Agent {
public:
    Agent(GDBusConnection* connection, const std::string &objectPath);
    ~Agent();

    void registerAgent();

private:
    static const char* introspectionXml;
    static void handleMethodCall(GDBusConnection *connection,
                                 const gchar *sender,
                                 const gchar *object_path,
                                 const gchar *interface_name,
                                 const gchar *method_name,
                                 GVariant *parameters,
                                 GDBusMethodInvocation *invocation,
                                 gpointer user_data);

    GDBusConnection* connection;
    std::string objectPath;
    guint registrationId;
    GDBusNodeInfo* nodeInfo;
};