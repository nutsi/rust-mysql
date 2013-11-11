use mysql_com::NET;
use mysql_ot::{LIST, MEM_ROOT, st_mysql_options};
use std::libc::{c_uint, c_void, c_schar, c_ulong, c_uchar, c_ulonglong};
use std::ptr;

type c_bool = c_schar;

type mysql_status = c_uint;
type st_mysql_methods = c_void;

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
pub type MYSQL = st_mysql;


#[link_args = "-L/usr/lib/mysql/ -lmysqlclient"]
extern "C" {
    fn mysql_init(mysql: *MYSQL) -> *mut MYSQL;
    fn mysql_real_connect(mysql: *mut MYSQL, host: *c_schar,
                          user: *c_schar, passwd: *c_schar,
                          db: *c_schar, port: c_uint,
                          unix_socket: *c_schar,client_flag: c_ulong)
                          -> *mut MYSQL;
}

#[fixed_stack_segment]
pub fn create() -> Option<*mut MYSQL> {
    unsafe {
        let toto: *MYSQL = ptr::null();
        let titi = mysql_init(toto);
        if ptr::is_null(titi) {
            return None; }
        return Some(titi);
    }
}

#[fixed_stack_segment]
pub fn real_connect(mut mysql: *mut MYSQL, host: &str, user: &str, passwd: &str,
                    db: &str, port: c_uint) -> Option<*mut MYSQL> {
    let unix_sock: *c_schar = ptr::null();
    host.to_c_str().with_ref(|ho|
                             user.to_c_str().with_ref(|us|
                                                      passwd.to_c_str().with_ref(|pass| db.to_c_str().with_ref(|d| unsafe {
                        mysql = mysql_real_connect(mysql, ho, us, pass, d, port,
                                                   unix_sock, 0); }))));
    if ptr::is_null(mysql) {
        return None; }
    return Some(mysql);
}



