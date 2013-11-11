extern mod mysql;

fn main() {
    let opt = mysql::mysql::create();
 let ret = match mysql::mysql::real_connect(opt.unwrap(), "127.0.0.1", "****", "****",
                                        "table_name", 0) {
        None => { println("nop bad luck"); return ; }
        Some(x) => { println("C'est tout bon"); x }
    };
}
