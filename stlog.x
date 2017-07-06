SECTIONS
{
  .stlog 0 (INFO) : {
    _sstlog_trace = .;
    *(.stlog.trace*);
    _estlog_trace = .;

    _sstlog_debug = .;
    *(.stlog.debug*);
    _estlog_debug = .;

    _sstlog_info = .;
    *(.stlog.info*);
    _estlog_info = .;

    _sstlog_warn = .;
    *(.stlog.warn*);
    _estlog_warn = .;

    _sstlog_error = .;
    *(.stlog.error*);
    _estlog_error = .;
  }
}
