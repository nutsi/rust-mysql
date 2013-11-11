use std::libc::{c_void, c_uint, c_schar, c_ulong, c_int};
use std::libc::types::os::arch::c95::size_t;

type c_bool = c_schar;
type st_mysql_options_extention = c_void;
type st_dynamic_array = c_void;
type enum_mysql_option = c_uint;

struct st_list {
    prev: *c_void,
    next: *c_void,
    data: *c_void,
}

struct st_used_mem {
    next: *c_void,
    left: c_uint,
    size: c_uint
}

struct st_mem_root {
    free: *USED_MEM,
    used: *USED_MEM,
    pre_alloc: *USED_MEM,
    min_malloc: size_t,
    block_size: size_t,
    block_num: c_uint,
    first_block_usage: c_uint,
    error_handler: *c_void
}

pub struct st_mysql_options {
    connect_timeout: c_uint,
    read_timeout: c_uint,
    write_timeout: c_uint,
    port: c_uint,
    protocol: c_uint,
    client_flag: c_ulong,
    host: *mut c_schar,
    user: *mut c_schar,
    password: *mut c_schar,
    unix_socket: *mut c_schar,
    db: *mut c_schar,
    init_commands: *mut st_dynamic_array,
    my_cnf_file: *mut c_schar,
    my_cnf_group: *mut c_schar,
    charset_dir: *mut c_schar,
    charset_name: *mut c_schar,
    ssl_key: *mut c_schar,
    ssl_cert: *mut c_schar,
    ssl_ca: *mut c_schar,
    ssl_capath: *mut c_schar,
    ssl_cipher: *mut c_schar,
    shared_memory_base_name: *mut c_schar,
    max_allowed_packet: c_ulong,
    use_ssl: c_bool,
    compress: c_bool,
    named_pipe: c_bool,
    unused1: c_bool,
    unused2: c_bool,
    unused3: c_bool,
    unused4: c_bool,
    methods_to_use: enum_mysql_option,
    client_ip: *mut c_schar,
    secure_auth: c_bool,
    report_data_truncation: c_bool,
    local_infile_init: extern "C" fn
                           (arg1: *mut *mut c_void, arg2: *c_schar,
                            arg3: *mut c_void) -> c_int,
    local_infile_read: extern "C" fn
                           (arg1: *mut c_void, arg2: *mut c_schar,
                            arg3: c_uint) -> c_int,
    local_infile_end: extern "C" fn(arg1: *mut c_void),
    local_infile_error: extern "C" fn
                            (arg1: *mut c_void, arg2: *mut c_schar,
                             arg3: c_uint) -> c_int,
    local_infile_userdata: *mut c_void,
    extension: *mut st_mysql_options_extention,
}

pub type USED_MEM = st_used_mem;
pub type LIST = st_list;
pub type MEM_ROOT = st_mem_root;

