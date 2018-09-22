SECTIONS
{
  .stlog 0 (INFO) : {
    __stlog_error_start__ = .;
    *(.stlog.error);
    __stlog_error_end__ = .;

    __stlog_warn_start__ = .;
    *(.stlog.warn);
    __stlog_warn_end__ = .;

    __stlog_info_start__ = .;
    *(.stlog.info);
    __stlog_info_end__ = .;

    __stlog_debug_start__ = .;
    *(.stlog.debug);
    __stlog_debug_end__ = .;

    __stlog_trace_start__ = .;
    *(.stlog.trace);
    __stlog_trace_end__ = .;
  }
}

ASSERT(__stlog_trace_end__ < 256, "
ERROR(stlog): stlog only supports up to 256 different strings at the moment.");
