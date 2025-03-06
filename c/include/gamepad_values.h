#ifndef GAMEPAD_VALUES_H
#define GAMEPAD_VALUES_H

#include <stdint.h>
#include <stddef.h>

typedef struct
{
    /* 1-bit values (stored as uint8_t but only 0 or 1 are used) */
    uint8_t Btn10;
    uint8_t Btn11;
    uint8_t Btn12;
    uint8_t Btn13;
    uint8_t Btn14;
    uint8_t Btn15;
    uint8_t Btn16;
    uint8_t Btn17;
    /* 8-bit values */
    uint8_t Slider0;
    uint8_t AxisX0;
    uint8_t AxisY0;
    uint8_t AxisZ0;
    uint8_t AxisRx0;
    uint8_t AxisRy0;
    uint8_t AxisRz0;
    uint8_t AxisVx0;
} GamepadValues;

/* Create a new instance (allocated on the heap) with all fields set to zero */
GamepadValues *gamepad_values_new(void);
/* Free an instance */
void gamepad_values_free(GamepadValues *vals);

/* Setters for button fields (masking to 1 bit) */
void gamepad_values_set_Btn10(GamepadValues *vals, uint8_t value);
void gamepad_values_set_Btn11(GamepadValues *vals, uint8_t value);
void gamepad_values_set_Btn12(GamepadValues *vals, uint8_t value);
void gamepad_values_set_Btn13(GamepadValues *vals, uint8_t value);
void gamepad_values_set_Btn14(GamepadValues *vals, uint8_t value);
void gamepad_values_set_Btn15(GamepadValues *vals, uint8_t value);
void gamepad_values_set_Btn16(GamepadValues *vals, uint8_t value);
void gamepad_values_set_Btn17(GamepadValues *vals, uint8_t value);

/* Setters for other fields (masking to 8 bits) */
void gamepad_values_set_Slider0(GamepadValues *vals, uint8_t value);
void gamepad_values_set_AxisX0(GamepadValues *vals, uint8_t value);
void gamepad_values_set_AxisY0(GamepadValues *vals, uint8_t value);
void gamepad_values_set_AxisZ0(GamepadValues *vals, uint8_t value);
void gamepad_values_set_AxisRx0(GamepadValues *vals, uint8_t value);
void gamepad_values_set_AxisRy0(GamepadValues *vals, uint8_t value);
void gamepad_values_set_AxisRz0(GamepadValues *vals, uint8_t value);
void gamepad_values_set_AxisVx0(GamepadValues *vals, uint8_t value);

/* Generate a report from the values.
 * The report is 10 bytes:
 *   Byte0 is Report ID (fixed to 0x01)
 *   Byte1 contains the 8 button bits (Btn10 is the MSB, Btn17 the LSB)
 *   Bytes 2..9 contain, in order, Slider0, AxisX0, AxisY0, AxisZ0,
 *           AxisRx0, AxisRy0, AxisRz0, AxisVx0.
 * The returned buffer is allocated on the heap; caller must free it.
 * The actual length (10) is stored in *length.
 */
uint8_t *gamepad_values_get_report(GamepadValues *vals, size_t *length);

/* Return a pointer to a constant report map (descriptor) and its length.
 * The report map describes the HID report layout.
 * The returned pointer must not be modified or freed.
 */
const uint8_t *gamepad_values_get_report_map(size_t *length);

#endif // GAMEPAD_VALUES_H