use mysql_com::NET;
use mysql_ot::{LIST, MEM_ROOT, st_mysql_options, enum_mysql_option};
use std::libc::{c_uint, c_void, c_schar, c_ulong, c_uchar, c_ulonglong, c_int};
use std::ptr;
use std::vec;
use std::str;

type c_bool = c_schar;

type mysql_status = c_uint;
type st_mysql_methods = c_void;

enum enum_field_types {
}

pub type MYSQL_ROW = * * c_schar;

struct st_mysql_data {
    data: *mut MYSQL_ROWS,
    embedded_info: *mut c_void,
    alloc: MEM_ROOT,
    rows: c_ulonglong,
    fields: c_uint,
    extension: *mut c_void,
}
pub type MYSQL_DATA = st_mysql_data;

struct st_mysql_rows {
    next: *mut st_mysql_rows,
    data: MYSQL_ROW,
    length: c_ulong
}

struct st_mysql_res {
    row_count: c_ulonglong,
    fields: *mut MYSQL_FIELD,
    data: *mut MYSQL_DATA,
    data_cursor: *mut MYSQL_ROWS,
    lengths: *mut c_ulong,
    handle: *mut MYSQL,
    methods: *st_mysql_methods,
    row: MYSQL_ROW,
    current_row: MYSQL_ROW,
    field_alloc: MEM_ROOT,
    field_count: c_uint,
    current_field: c_uint,
    eof: c_bool,
    unbuffered_fetch_cancelled: c_bool,
    extension: *mut c_void,
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
type MYSQL_RES = st_mysql_res;
type MYSQL_ROWS = st_mysql_rows;


#[link_args = "-L/usr/lib/mysql/ -lmysqlclient"]
extern "C" {
    fn mysql_init(mysql: *MYSQL) -> *mut MYSQL;
    fn mysql_real_connect(mysql: *mut MYSQL, host: *c_schar,
                          user: *c_schar, passwd: *c_schar,
                          db: *c_schar, port: c_uint,
                          unix_socket: *c_schar,client_flag: c_ulong)
                          -> *mut MYSQL;
    fn mysql_close(mysql: *mut MYSQL) -> ();
    fn mysql_affected_rows(mysql: *mut MYSQL) -> c_ulonglong;
    fn mysql_autocommit(mysql: *mut MYSQL, mode: c_bool) -> c_bool;
    fn mysql_change_user(mysql: *mut MYSQL, user: *c_schar,
                         passwd: *c_schar, db: *c_schar) -> c_bool;
    fn mysql_character_set_name(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_commit(mysql: *mut MYSQL) -> c_bool;
    fn mysql_debug(debug: *c_schar);
    fn mysql_dump_debug_info(mysql: *mut MYSQL) -> c_int;
    fn mysql_errno(mysql: *mut MYSQL) -> c_uint;
    fn mysql_error(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_field_count(mysql: *mut MYSQL) -> c_uint;
    fn mysql_get_client_info() -> *c_schar;
    fn mysql_get_client_version() -> c_ulong;
    fn mysql_get_host_info(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_get_proto_info(mysql: *mut MYSQL) -> c_uint;
    fn mysql_get_server_info(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_get_server_version(mysql: *mut MYSQL) -> c_ulong;
    fn mysql_get_ssl_cipher(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_hex_string(to: *mut c_schar, from: *c_schar,
                        from_length: c_ulong) -> c_ulong;
    fn mysql_info(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_insert_id(mysql: *mut MYSQL) -> c_ulonglong;
    fn mysql_kill(mysql: *mut MYSQL, pid: c_ulong) -> c_int;
    fn mysql_server_end();
    fn mysql_library_end();
    fn mysql_more_results(mysql: *mut MYSQL) -> c_bool;
    fn mysql_next_result(mysql: *mut MYSQL) -> c_int;
    fn mysql_options(mysql: *mut MYSQL, option: enum_mysql_option,
                     arg: *c_void) -> c_int;
    fn mysql_ping(mysql: *mut MYSQL) -> c_int;
    fn mysql_query(mysql: *mut MYSQL, q: *c_schar) -> c_int;
    fn mysql_real_escape_string(mysql: *mut MYSQL, to: *mut c_schar,
                                from: *c_schar, length: c_ulong) -> c_ulong;
    fn mysql_real_query(mysql: *mut MYSQL, q: *c_schar, length: c_ulong)
                        -> c_int;
    fn mysql_refresh(mysql: *mut MYSQL, refresh_options: c_uint) -> c_int;
    fn mysql_rollback(mysql: *mut MYSQL) -> c_bool;
    fn mysql_select_db(mysql: *mut MYSQL, db: *c_schar) -> c_int;
    fn mysql_set_character_set(mysql: *mut MYSQL, csname: *c_schar) -> c_int;
    fn mysql_set_local_infile_default(mysql: *mut MYSQL);
    fn mysql_sqlstate(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_ssl_set(mysql: *mut MYSQL, key: *c_schar, cert: *c_schar,
                     ca: *c_schar, capath: *c_schar, cipher: *c_schar) -> c_bool;
    fn mysql_stat(mysql: *mut MYSQL) -> *c_schar;
    fn mysql_use_result(mysql: *mut MYSQL) -> *mut MYSQL_RES;
    fn mysql_num_fields(res: *mut MYSQL_RES) -> c_uint;
    fn mysql_fetch_row(res: *mut MYSQL_RES) -> MYSQL_ROW;
    fn mysql_fetch_lengths(res: *mut MYSQL_RES) -> *c_ulong;
    fn mysql_free_result(res: *mut MYSQL_RES);
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

#[fixed_stack_segment]
pub fn close(mysql: *mut MYSQL) -> () {
    unsafe { mysql_close(mysql); }
}

#[fixed_stack_segment]
pub fn query(mysql: *mut MYSQL, query: &str) -> int {
    let mut t: c_int = 0;
    query.to_c_str().with_ref(|q | unsafe { t = mysql_query(mysql, q); });
    return t as int;
}

#[fixed_stack_segment]
pub fn use_result(mysql: *mut MYSQL) -> (int, ~[~str]) {
    let mut tab: ~[~str] = ~[];
    unsafe {
        let res = mysql_use_result(mysql);
        let t = mysql_num_fields(res) as int;
        println!("{:i}", t);

        loop {
            let row = mysql_fetch_row(res);
            if ptr::is_not_null(row) {
                let length = mysql_fetch_lengths(res);
                let test = vec::raw::from_buf_raw(length, t as uint);
                for j in range(0, t) {
                    for e in range(0, t) {
                        let n = ptr::offset(row, e as int);
                        let chaine = str::raw::from_c_str(*n);
                        tab.push(chaine.clone());
                    }
                    println!("{:i}", test[j] as int);
                }
            }
            else {
                break;
            }
        }
        mysql_free_result(res);
    }
    (1, tab)
}

#[fixed_stack_segment]
pub fn errno(mysql: *mut MYSQL) -> uint {
    unsafe {
        mysql_errno(mysql) as uint
    }
}

#[fixed_stack_segment]
pub fn error(mysql: *mut MYSQL) -> ~str {
    unsafe {
        let ret = str::raw::from_c_str(mysql_error(mysql));
        return ret;
    }
}

