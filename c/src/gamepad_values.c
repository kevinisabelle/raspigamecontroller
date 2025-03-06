#include <gamepad_values.h>
#include <stdlib.h>
#include <string.h>

/* Create a new GamepadValues instance (all fields initialized to zero) */
GamepadValues *gamepad_values_new(void)
{
    GamepadValues *vals = malloc(sizeof(GamepadValues));
    if (vals)
    {
        memset(vals, 0, sizeof(GamepadValues));
    }
    return vals;
}

void gamepad_values_free(GamepadValues *vals)
{
    free(vals);
}

/* Button setters (masking to 1 bit) */
void gamepad_values_set_Btn10(GamepadValues *vals, uint8_t value) { vals->Btn10 = value & 0x1; }
void gamepad_values_set_Btn11(GamepadValues *vals, uint8_t value) { vals->Btn11 = value & 0x1; }
void gamepad_values_set_Btn12(GamepadValues *vals, uint8_t value) { vals->Btn12 = value & 0x1; }
void gamepad_values_set_Btn13(GamepadValues *vals, uint8_t value) { vals->Btn13 = value & 0x1; }
void gamepad_values_set_Btn14(GamepadValues *vals, uint8_t value) { vals->Btn14 = value & 0x1; }
void gamepad_values_set_Btn15(GamepadValues *vals, uint8_t value) { vals->Btn15 = value & 0x1; }
void gamepad_values_set_Btn16(GamepadValues *vals, uint8_t value) { vals->Btn16 = value & 0x1; }
void gamepad_values_set_Btn17(GamepadValues *vals, uint8_t value) { vals->Btn17 = value & 0x1; }

/* Setters for other fields (masking to 8 bits) */
void gamepad_values_set_Slider0(GamepadValues *vals, uint8_t value) { vals->Slider0 = value & 0xFF; }
void gamepad_values_set_AxisX0(GamepadValues *vals, uint8_t value) { vals->AxisX0 = value & 0xFF; }
void gamepad_values_set_AxisY0(GamepadValues *vals, uint8_t value) { vals->AxisY0 = value & 0xFF; }
void gamepad_values_set_AxisZ0(GamepadValues *vals, uint8_t value) { vals->AxisZ0 = value & 0xFF; }
void gamepad_values_set_AxisRx0(GamepadValues *vals, uint8_t value) { vals->AxisRx0 = value & 0xFF; }
void gamepad_values_set_AxisRy0(GamepadValues *vals, uint8_t value) { vals->AxisRy0 = value & 0xFF; }
void gamepad_values_set_AxisRz0(GamepadValues *vals, uint8_t value) { vals->AxisRz0 = value & 0xFF; }
void gamepad_values_set_AxisVx0(GamepadValues *vals, uint8_t value) { vals->AxisVx0 = value & 0xFF; }

/* Generate a report.
 * The report layout (10 bytes):
 *   Byte 0: Report ID (0x01)
 *   Byte 1: Buttons (bit7: Btn10, ..., bit0: Btn17)
 *   Bytes 2..9: Slider0, AxisX0, AxisY0, AxisZ0, AxisRx0, AxisRy0, AxisRz0, AxisVx0
 */
uint8_t *gamepad_values_get_report(GamepadValues *vals, size_t *length)
{
    *length = 10;
    uint8_t *report = malloc(*length);
    if (!report)
        return NULL;
    report[0] = 0x01;
    report[1] = ((vals->Btn10 & 0x1) << 7) |
                ((vals->Btn11 & 0x1) << 6) |
                ((vals->Btn12 & 0x1) << 5) |
                ((vals->Btn13 & 0x1) << 4) |
                ((vals->Btn14 & 0x1) << 3) |
                ((vals->Btn15 & 0x1) << 2) |
                ((vals->Btn16 & 0x1) << 1) |
                ((vals->Btn17 & 0x1) << 0);
    report[2] = vals->Slider0;
    report[3] = vals->AxisX0;
    report[4] = vals->AxisY0;
    report[5] = vals->AxisZ0;
    report[6] = vals->AxisRx0;
    report[7] = vals->AxisRy0;
    report[8] = vals->AxisRz0;
    report[9] = vals->AxisVx0;
    return report;
}

/* The report map is constant. Its content describes the layout of the HID report.
   This array matches the Python version from your GamepadValues.py.
*/
static const uint8_t report_map[] = {
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x05, // Usage (Gamepad)
    0xA1, 0x01, // Collection (Application)
    0x85, 0x01, // Report ID 1
    0x05, 0x09, // Usage Page (Button)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x08, // Usage Maximum (8)
    0x15, 0x00, // Logical Minimum (0)
    0x25, 0x01, // Logical Maximum (1)
    0x75, 0x01, // Report Size (1)
    0x95, 0x08, // Report Count (8)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x36, // Usage (Slider)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x01, // Usage Maximum (1)
    0x15, 0x00, // Logical Minimum (0)
    0x25, 0xFF, // Logical Maximum (255)
    0x35, 0x00, // Physical Minimum (0)
    0x45, 0xFF, // Physical Maximum (255)
    0x75, 0x08, // Report Size (8)
    0x95, 0x01, // Report Count (1)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x30, // Usage (X)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x01, // Usage Maximum (1)
    0x15, 0x81, // Logical Minimum (-127)
    0x25, 0x7F, // Logical Maximum (127)
    0x35, 0x81, // Physical Minimum (-127)
    0x45, 0x7F, // Physical Maximum (127)
    0x75, 0x08, // Report Size (8)
    0x95, 0x01, // Report Count (1)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x31, // Usage (Y)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x01, // Usage Maximum (1)
    0x15, 0x00, // Logical Minimum (0)
    0x25, 0xFF, // Logical Maximum (255)
    0x35, 0x00, // Physical Minimum (0)
    0x45, 0xFF, // Physical Maximum (255)
    0x75, 0x08, // Report Size (8)
    0x95, 0x01, // Report Count (1)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x32, // Usage (Z)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x01, // Usage Maximum (1)
    0x15, 0x00, // Logical Minimum (0)
    0x25, 0xFF, // Logical Maximum (255)
    0x35, 0x00, // Physical Minimum (0)
    0x45, 0xFF, // Physical Maximum (255)
    0x75, 0x08, // Report Size (8)
    0x95, 0x01, // Report Count (1)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x33, // Usage (Rx)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x01, // Usage Maximum (1)
    0x15, 0x00, // Logical Minimum (0)
    0x25, 0xFF, // Logical Maximum (255)
    0x35, 0x00, // Physical Minimum (0)
    0x45, 0xFF, // Physical Maximum (255)
    0x75, 0x08, // Report Size (8)
    0x95, 0x01, // Report Count (1)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x34, // Usage (Ry)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x01, // Usage Maximum (1)
    0x15, 0x00, // Logical Minimum (0)
    0x25, 0xFF, // Logical Maximum (255)
    0x35, 0x00, // Physical Minimum (0)
    0x45, 0xFF, // Physical Maximum (255)
    0x75, 0x08, // Report Size (8)
    0x95, 0x01, // Report Count (1)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0x05, 0x01, // Usage Page (Generic Desktop Controls)
    0x09, 0x35, // Usage (Rz)
    0x19, 0x01, // Usage Minimum (1)
    0x29, 0x01, // Usage Maximum (1)
    0x15, 0x00, // Logical Minimum (0)
    0x25, 0xFF, // Logical Maximum (255)
    0x35, 0x00, // Physical Minimum (0)
    0x45, 0xFF, // Physical Maximum (255)
    0x75, 0x08, // Report Size (8)
    0x95, 0x01, // Report Count (1)
    0x81, 0x02, // Input (Data, Variable, Absolute)
    0xC0        // End Collection
};

const uint8_t *gamepad_values_get_report_map(size_t *length)
{
    *length = sizeof(report_map);
    return report_map;
}