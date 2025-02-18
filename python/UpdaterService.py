import threading
import time
from GamepadValues import GamepadValues1
from HidServiceImpl import Application
import random
from Hardware import read_joystick, read_slider, read_rotary, read_pot, read_button

class GamepadUpdater:
    def __init__(self, gamepad_def : GamepadValues1, app: Application, poll_interval=1):
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

    def _update_control(self, getter, setter, read_func, idx) -> bool:
        new_val = read_func(idx)
        if getter() != new_val:
            setter(new_val)
            return True
        return False

    def _update_gamepad_controls(self) -> bool:
        """Polls hardware for each control and updates its value."""
        hasChanged = False

        # Using one-liner calls for each control update.
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Joystick0, self.gamepad_def.set_Joystick0, read_joystick, 0)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Joystick1, self.gamepad_def.set_Joystick1, read_joystick, 1)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Joystick2, self.gamepad_def.set_Joystick2, read_joystick, 2)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Joystick3, self.gamepad_def.set_Joystick3, read_joystick, 3)

        # hasChanged |= self._update_control(lambda: self.gamepad_def.Slider0, self.gamepad_def.set_Slider0, read_slider, 0)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Slider1, self.gamepad_def.set_Slider1, read_slider, 1)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Slider2, self.gamepad_def.set_Slider2, read_slider, 2)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Slider3, self.gamepad_def.set_Slider3, read_slider, 3)

        # hasChanged |= self._update_control(lambda: self.gamepad_def.Rotary0, self.gamepad_def.set_Rotary0, read_rotary, 0)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Rotary1, self.gamepad_def.set_Rotary1, read_rotary, 1)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Rotary2, self.gamepad_def.set_Rotary2, read_rotary, 2)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Rotary3, self.gamepad_def.set_Rotary3, read_rotary, 3)

        # hasChanged |= self._update_control(lambda: self.gamepad_def.Pot0, self.gamepad_def.set_Pot0, read_pot, 0)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Pot1, self.gamepad_def.set_Pot1, read_pot, 1)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Pot2, self.gamepad_def.set_Pot2, read_pot, 2)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Pot3, self.gamepad_def.set_Pot3, read_pot, 3)

        hasChanged |= self._update_control(lambda: self.gamepad_def.Button0, self.gamepad_def.set_Button0, read_button, 0)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Button1, self.gamepad_def.set_Button1, read_button, 1)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Button2, self.gamepad_def.set_Button2, read_button, 2)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Button3, self.gamepad_def.set_Button3, read_button, 3)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Button4, self.gamepad_def.set_Button4, read_button, 4)
        # hasChanged |= self._update_control(lambda: self.gamepad_def.Button5, self.gamepad_def.set_Button5, read_button, 5)

        return hasChanged