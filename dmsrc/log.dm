#define rustg_log_write(fname, text) RUSTG_CALL(RUST_G, "log_write")(fname, text)
/proc/rustg_log_close_all() return RUSTG_CALL(RUST_G, "log_close_all")()
