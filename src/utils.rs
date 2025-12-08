use std::sync::{Arc, Mutex};
use tracing::error;

pub mod pvz;

#[macro_export]
macro_rules! pause {
    () => {
        use std::io::{self, Read};
        let _ = io::stdin().read(&mut [0u8]);
    };
    ($($args:tt)*) => {
        use std::io::{self, Read};
        println!($($args)*);
        let _ = io::stdin().read(&mut [0u8]);
    };
}

pub fn get_arc_mutex<T: std::fmt::Debug>(arc: &'_ Arc<Mutex<T>>) -> std::sync::MutexGuard<'_, T> {
    match arc.lock() {
        Ok(guard) => guard,
        Err(e) => {
            error!("{} 锁错误: {}", std::any::type_name::<T>(), e);
            panic!("{} 锁错误: {}", std::any::type_name::<T>(), e)
        }
    }
}
