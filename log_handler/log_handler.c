#include <stdarg.h>
#include <libseat.h>

#include <stdio.h>
#include <stdlib.h>

#include "log_handler.h"

static void noop_handler(enum libseat_log_level level, const char *msg, const void *userdata)
{
}

static LogHandler current_log_handler = noop_handler;
static const void *current_userdata = NULL;

int str_length(const char *format, va_list args)
{
    va_list argcopy;
    va_copy(argcopy, args);
    int retval = vsnprintf(NULL, 0, format, argcopy);
    va_end(argcopy);
    return retval;
}

static void formatter_handler(enum libseat_log_level level, const char *fmt, va_list args)
{
    if (level > LIBSEAT_LOG_LEVEL_LAST)
    {
        level = LIBSEAT_LOG_LEVEL_LAST;
    }

    int length = str_length(fmt, args);

    if (length >= 0)
    {
        // +1 for null terminator
        length += 1;

        char *buffer = (char *)malloc(length * sizeof(char));

        vsnprintf(buffer, length, fmt, args);

        current_log_handler(level, buffer, current_userdata);

        free(buffer);
    }
}

void init_preformatted_log_handler(LogHandler handler, const void *userdata)
{
    current_userdata = userdata;
    current_log_handler = handler;
    libseat_set_log_handler(formatter_handler);
}

void drop_preformatted_log_handler()
{
    libseat_set_log_handler(NULL);
    current_userdata = NULL;
    current_log_handler = noop_handler;
}