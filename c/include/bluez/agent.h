#ifndef AGENT_H
#define AGENT_H

#include <gio/gio.h>

/* Opaque type for our Agent */
typedef struct _Agent Agent;

/* Create a new Agent on the given D-Bus connection at object_path */
Agent *agent_new(GDBusConnection *connection, const char *object_path);

/* Register the agent on the bus */
void agent_register(Agent *agent);

/* Free the Agent */
void agent_free(Agent *agent);

#endif // AGENT_H