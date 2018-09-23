SECTIONS
{
  .stlog 0 (INFO) : {
    *(.stlog.error);

    __stlog_warning_start__ = .;
    *(.stlog.warn);

    __stlog_info_start__ = .;
    *(.stlog.info);

    __stlog_debug_start__ = .;
    *(.stlog.debug);

    __stlog_trace_start__ = .;
    *(.stlog.trace);
  }
}

ASSERT(SIZEOF(.stlog) < 256, "
ERROR(stlog): stlog only supports up to 256 different strings at the moment.");
