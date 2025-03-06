import threading
import time
from GamepadValues import GamepadValues1
from HidServiceImpl import Application
from Hardware import read_slider, read_button, read_slider_middle

class GamepadUpdater:
    def __init__(self, gamepad_def : GamepadValues1, app: Application, poll_interval=0.05):
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

        hasChanged |= self._update_control(lambda: self.gamepad_def.Slider0, self.gamepad_def.set_Slider0, read_slider, 0)
        hasChanged |= self._update_control(lambda: self.gamepad_def.AxisX0, self.gamepad_def.set_AxisX0, read_slider_middle, 1)
        hasChanged |= self._update_control(lambda: self.gamepad_def.AxisY0, self.gamepad_def.set_AxisY0, read_slider, 2)
        hasChanged |= self._update_control(lambda: self.gamepad_def.AxisZ0, self.gamepad_def.set_AxisZ0, read_slider, 3)
        hasChanged |= self._update_control(lambda: self.gamepad_def.AxisRx0, self.gamepad_def.set_AxisRx0, read_slider, 4)
        hasChanged |= self._update_control(lambda: self.gamepad_def.AxisRy0, self.gamepad_def.set_AxisRy0, read_slider, 5)
        hasChanged |= self._update_control(lambda: self.gamepad_def.AxisRz0, self.gamepad_def.set_AxisRz0, read_slider, 6)
        hasChanged |= self._update_control(lambda: self.gamepad_def.AxisVx0, self.gamepad_def.set_AxisVx0, read_slider, 7)
        
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn10, self.gamepad_def.set_Btn10, read_button, 0)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn11, self.gamepad_def.set_Btn11, read_button, 1)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn12, self.gamepad_def.set_Btn12, read_button, 2)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn13, self.gamepad_def.set_Btn13, read_button, 3)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn14, self.gamepad_def.set_Btn14, read_button, 4)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn15, self.gamepad_def.set_Btn15, read_button, 5)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn16, self.gamepad_def.set_Btn16, read_button, 6)
        hasChanged |= self._update_control(lambda: self.gamepad_def.Btn17, self.gamepad_def.set_Btn17, read_button, 7)

        return hasChanged