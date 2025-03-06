#ifndef UPDATER_SERVICE_H
#define UPDATER_SERVICE_H

#include <stdbool.h>
#include "gamepad_values.h"
#include "hid_service_impl.h" // Assumes an Application type with notify_hid_report()

typedef struct _GamepadUpdater
{
    GamepadValues *gamepad_values;
    Application *app;
    double poll_interval; // in seconds
    bool running;
    // thread handle (using POSIX threads)
    pthread_t thread;
} GamepadUpdater;

/* Create a new updater instance.
 * gamepad: Pointer to a GamepadValues instance.
 * app: Pointer to an Application instance.
 * poll_interval: Interval (in seconds) between hardware polls.
 * Returns a pointer to an allocated updater.
 */
GamepadUpdater *gamepad_updater_new(GamepadValues *gamepad, Application *app, double poll_interval);

/* Starts the polling thread. */
void gamepad_updater_start(GamepadUpdater *updater);

/* Stops the polling thread and waits for its termination. */
void gamepad_updater_stop(GamepadUpdater *updater);

/* Free the updater instance. */
void gamepad_updater_free(GamepadUpdater *updater);

#endif // UPDATER_SERVICE_H