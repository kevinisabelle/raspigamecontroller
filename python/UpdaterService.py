import threading
import time
from HidGamepadReport import GamepadDefinition
from HidServiceImpl import Application
import random

class GamepadUpdater:
    def __init__(self, gamepad_def : GamepadDefinition, app: Application, poll_interval=0.05):
        """
        Initialize the updater with a GamepadDefinition instance.
        
        :param gamepad_def: The gamepad definition object whose controls will be updated.
        :param poll_interval: Time in seconds between hardware polls.
        """
        self.gamepad_def = gamepad_def
        self.poll_interval = poll_interval
        self._running = False
        self.thread = None
        self.app = app

    def start(self):
        """Starts the background polling thread."""
        if not self._running:
            self._running = True
            self.thread = threading.Thread(target=self._poll_loop, daemon=True)
            self.thread.start()

    def stop(self):
        """Stops the background polling thread."""
        self._running = False
        if self.thread:
            self.thread.join()

    def _poll_loop(self):
        """Internal method: loop that polls hardware and updates controls."""
        while self._running:
            hasChanged = self._update_gamepad_controls()
            if hasChanged:
                self.app.notify_hid_report()

            time.sleep(self.poll_interval)

    def _update_gamepad_controls(self) -> bool:
        """Polls hardware for each control and updates its value."""
        for control in self.gamepad_def.controls:
            old_val = control.value
            new_val = self._read_hardware_value(control)
            if new_val != old_val:
                control.set_value(new_val)
                return True
        return False
            
    def _read_hardware_value(self, control):
        """
        Replace this method with your actual hardware polling code.
        This example just returns the current value for demonstration.
        
        :param control: The GamepadControl instance to poll.
        :return: The new value for the control.
        """
        # TODO: Insert hardware reading logic here.

        # Random value from 1 to 6
        return random.randint(1, 63)
