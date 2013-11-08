struct MYSQL {
    connector_fd: ~std::libc::types::os::arch::c95::c_char
}

#[link_args="-L/usr/lib/mysql/ -lmysqlclient"]
extern {

    fn mysql_init(my: MYSQL) -> MYSQL;

}

#[fixed_stack_segment]
fn main() {
    let toto = MYSQL { connector_fd: ~0};
    unsafe { mysql_init(toto); }
}
