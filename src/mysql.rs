use mysql_com::NET;
use mysql_ot::{LIST, MEM_ROOT};
use std::libc::{c_uint, c_void, c_schar, c_ulong, c_uchar, c_int, c_ulonglong};
use std::ptr;

type c_bool = c_schar;

type st_mysql_options_extention = c_void;
type mysql_status = c_uint;
type st_mysql_methods = c_void;
type st_dynamic_array = c_void;

struct st_mysql_options {
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

enum enum_field_types {
}

struct st_mysql_field {
    name: *mut c_schar,
    org_name: *mut c_schar,
    table: *mut c_schar,
    org_table: *mut c_schar,
    db: *mut c_schar,
    catalog: *mut c_schar,
    def: *mut c_schar,
    length: c_ulong,
    max_length: c_ulong,
    name_length: c_uint,
    org_name_length: c_uint,
    table_length: c_uint,
    org_table_length: c_uint,
    db_length: c_uint,
    catalog_length: c_uint,
    def_length: c_uint,
    flags: c_uint,
    decimals: c_uint,
    charsetnr: c_uint,
    _type: enum_field_types,
    extension: *mut c_void,
}

struct st_mysql {
    net: NET,
    connector_fd: *mut c_uchar,
    host: *mut c_schar,
    user: *mut c_schar,
    passwd: *mut c_schar,
    unix_socket: *mut c_schar,
    server_version: *mut c_schar,
    host_info: *mut c_schar,
    info: *mut c_schar,
    db: *mut c_schar,
    charset: *mut st_charset_info_set,
    fields: *mut MYSQL_FIELD,
    field_alloc: MEM_ROOT,
    affected_rows: c_ulonglong,
    insert_id: c_ulonglong,
    extra_info: c_ulonglong,
    thread_id: c_ulong,
    packet_length: c_ulong,
    port: c_uint,
    client_flag: c_ulong,
    server_capabilities: c_ulong,
    protocol_version: c_uint,
    field_count: c_uint,
    server_status: c_uint,
    server_language: c_uint,
    warning_count: c_uint,
    options: st_mysql_options,
    status: mysql_status,
    free_me: c_bool,
    reconnect: c_bool,
    scramble: [c_schar, ..21u],
    unused1: c_bool,
    unused2: *mut c_void,
    unused3: *mut c_void,
    unused4: *mut c_void,
    unused5: *mut c_void,
    stmts: *mut LIST,
    methods: *st_mysql_methods,
    thd: *mut c_void,
    unbuffered_fetch_owner: *mut c_bool,
    info_buffer: *mut c_schar,
    extension: *mut c_void,
}

type st_charset_info_set = c_void;
type MYSQL_FIELD = st_mysql_field;
type enum_mysql_option = c_uint;

pub type MYSQL = st_mysql;


#[link_args = "-L/usr/lib/mysql/ -lmysqlclient"]
extern "C" {
    fn mysql_init(mysql: *MYSQL) -> *mut MYSQL;
}

#[fixed_stack_segment]
pub fn init() -> Option<*mut MYSQL> {
    let mut titi: *mut MYSQL;
    unsafe {
        let toto: *MYSQL = ptr::null();
        titi = mysql_init(toto);
        if ptr::is_null(titi) {
            return None; }
        return Some(titi);
    }
}

