typedef void (*LogHandler)(enum libseat_log_level level, const char *msg, const void *userdata);

void init_preformated_log_handler(LogHandler handler, const void *userdata);
void drop_preformated_log_handler();