

    //--------4. Interior Mutability (Daxili dəyişən) ilə Singleton

    use once_cell::sync::OnceCell;
    use std::sync::Mutex;
    
    struct MySingleton {
        data: i32,
    }
    
    impl MySingleton {
        fn new() -> Self {
            MySingleton { data: 42 }
        }
    
        fn get_data(&self) -> i32 {
            self.data
        }
    }
    
    static INSTANCE: OnceCell<Mutex<MySingleton>> = OnceCell::new();
    
    fn get_instance() -> &'static Mutex<MySingleton> {
        INSTANCE.get_or_init(|| Mutex::new(MySingleton::new()))
    }
    
    fn main() {
        let instance = get_instance().lock().unwrap();
        println!("Singleton data: {}", instance.get_data());
    }
    



    //--------3. std::sync::Once ilə Singleton

    // use std::sync::{Once, OnceLock};

    // static INIT: Once = Once::new();
    // static mut INSTANCE: Option<MySingleton> = None;
    
    // struct MySingleton {
    //     data: i32,
    // }
    
    // impl MySingleton {
    //     fn new() -> Self {
    //         MySingleton { data: 42 }
    //     }
    
    //     fn get_instance() -> &'static MySingleton {
    //         unsafe {
    //             INIT.call_once(|| {
    //                 INSTANCE = Some(MySingleton::new());
    //             });
    //             INSTANCE.as_ref().unwrap()
    //         }
    //     }
    // }
    
    // fn main() {
    //     let instance = MySingleton::get_instance();
    //     println!("Singleton data: {}", instance.data);
    // }
    

   // ------- 2. Mutex və ya RwLock ilə Singleton ----------

//    use std::sync::{Arc, Mutex};
//    use std::thread;
   
//    struct MySingleton {
//        data: i32,
//    }
   
//    impl MySingleton {
//        fn new() -> Self {
//            MySingleton { data: 42 }
//        }
//    }
   
//    fn main() {
//        let singleton = Arc::new(Mutex::new(MySingleton::new()));
   
//        let handles: Vec<_> = (0..5)
//            .map(|_| {
//                let singleton_clone = Arc::clone(&singleton);
//                thread::spawn(move || {
//                    let mut instance = singleton_clone.lock().unwrap();
//                    instance.data += 1;
//                    println!("Updated data: {}", instance.data);
//                })
//            })
//            .collect();
   
//        for handle in handles {
//            handle.join().unwrap();
//        }
//    }
   



            /* ----------- once_cell::sync::Lazy ilə Singleton: -------------
R           ust 1.63+ versiyalarında lazy_static-in alternativi once_cell::sync::Lazy istifadə edilə bilər:
            */

// use once_cell::sync::Lazy;
// use std::sync::Mutex;

// static INSTANCE: Lazy<Mutex<MySingleton>> = Lazy::new(|| Mutex::new(MySingleton::new()));

// struct MySingleton {
//     data: i32,
// }

// impl MySingleton {
//     fn new() -> Self {
//         MySingleton { data: 42 }
//     }

//     fn get_data(&self) -> i32 {
//         self.data
//     }
// }

// fn main() {
//     let instance = INSTANCE.lock().unwrap();
//     println!("Singleton data: {}", instance.get_data());
// }


    //--------lazy_static! ilə Singleton:----------
// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref INSTANCE: Mutex<MySingleton> = Mutex::new(MySingleton::new());
// }

// struct MySingleton {
//     data: i32,
// }

// impl MySingleton {
//     fn new() -> Self {
//         MySingleton { data: 42 }
//     }

//     fn get_data(&self) -> i32 {
//         self.data
//     }
// }

// fn main() {
//     let instance = INSTANCE.lock().unwrap();
//     println!("Singleton data: {}", instance.get_data());
// }
