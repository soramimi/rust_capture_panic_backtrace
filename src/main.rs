use std::panic;
use std::sync::RwLock;
use std::backtrace::Backtrace;
use lazy_static::lazy_static;

// ref. https://qiita.com/kgtkr/items/a17827c4bb704f39c854

fn main2() {
    let src = "Hello, world";
    let mut v: Vec<char> = src.chars().collect();
    v[12] = '!';
    let s: String = v.iter().collect();
    println!("{}", s);
}

fn any_to_string(any: &dyn std::any::Any) -> String {
    if let Some(s) = any.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = any.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Any".to_string()
    }
}

lazy_static! {
    pub static ref LAST_PANIC_INFO: RwLock<Option<(String, Backtrace)>> = RwLock::new(None);
}

fn main() {
    let _default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        *self::LAST_PANIC_INFO.write().unwrap() = Some((
            format!("{}", panic_info),
            Backtrace::capture(),
        ));
        _default_panic(panic_info);
    }));
    if let Err(e) = panic::catch_unwind(move || {
        main2();
    }) {
        println!("--- error  ---");
        println!("{}", any_to_string(&*e));
        let info = self::LAST_PANIC_INFO.read().unwrap();
        let info = info.as_ref().unwrap();
        println!("--- message ---");
        println!("{}", info.0);
        println!("--- backtrace ---");
        println!("{}", info.1);
    }
}
