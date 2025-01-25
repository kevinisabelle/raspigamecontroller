#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include "lcd1602.h"

int main(int argc, char *argv[])
{
    int rc;
    rc = lcd1602Init(1, 0x27);
    if (rc)
    {
        printf("Initialization failed; aborting...\n");
        return 0;
    }
    lcd1602WriteString("BitBank LCD1602");
    lcd1602SetCursor(0, 1);
    lcd1602WriteString("ENTER to quit");
    lcd1602Control(1, 0, 1); // backlight on, underline off, blink block on
    getchar();
    lcd1602Shutdown();
    return 0;
} /* main() */