use jni::objects::*;
use jni::JNIEnv;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

static CLASS_CACHE: Lazy<RwLock<HashMap<String, JClass<'static>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static mut COUNT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

#[no_mangle]
pub extern "C" fn Java_Test_update(mut env: JNIEnv, _class: JClass, complex_object: JObject) -> () {
    let j_class = env
        .find_class("example/ComplexObject")
        .expect("class not found");
    let name_field = env
        .get_field_id(&j_class, "name", "Ljava/lang/String;")
        .expect("field not found");
    let _ = env.set_field_unchecked(
        &complex_object,
        name_field,
        JValueGen::Object(&env.new_string("test").expect("string not created")),
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_Test_alloc(mut env: JNIEnv<'static>, _: JClass) -> JObject<'static> {
    let j_class = env
        .find_class("example/ComplexObject")
        .expect("class not found");
    println!("class addr: {:p}", **j_class);
    let entity = env.alloc_object(&j_class).expect("alloc failed");
    let name_field = env
        .get_field_id(&j_class, "name", "Ljava/lang/String;")
        .expect("field not found");
    let _ = env.set_field_unchecked(
        &entity,
        name_field,
        JValueGen::Object(&env.new_string("test").expect("string not created")),
    );
    entity
}

#[no_mangle]
pub unsafe extern "C" fn Java_Test_allocWithCache(
    env: JNIEnv<'static>,
    _: JClass,
) -> JObject<'static> {
    let (mut env, j_class) = get_cached_class(env, "example/ComplexObject");
    let entity = env.alloc_object(&j_class).expect("alloc failed");
    let name_field = env
        .get_field_id(&j_class, "name", "Ljava/lang/String;")
        .expect("field not found");
    let _ = env.set_field_unchecked(
        &entity,
        name_field,
        JValueGen::Object(&env.new_string("test").expect("string not created")),
    );
    entity
}

fn get_cached_class(
    mut env: JNIEnv<'static>,
    class_name: &str,
) -> (JNIEnv<'static>, JClass<'static>) {
    unsafe {
        COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("reading cache {:?}", &COUNT.get_mut());
    };
    {
        let cache = CLASS_CACHE.read().unwrap();
        // first try from cache
        match cache.get(class_name) {
            Some(class) => {
                // let offset = unsafe { (COUNT.get_mut().clone() / 256 * 8) as isize };
                let offset = unsafe {
                    match COUNT.get_mut().clone() % 256 {
                        0 => 8,
                        _ => 0,
                    }
                };
                let ptr = class.as_raw().wrapping_byte_offset(offset.clone());
                let check_class = env.find_class(class_name).expect("class not found");
                println!("check addr: {:p}", check_class.into_raw());
                println!("read cache: {:p}", ptr);
                return (env, unsafe { JClass::from_raw(ptr) });
            }
            None => {
                println!("not hit cache");
            }
        }
    }
    let mut cache = CLASS_CACHE.write().unwrap();

    // or find from env
    let class = env.find_class(class_name).expect("class not found");
    let class_wrapper = unsafe { JClass::from_raw(class.as_raw()) };
    let ptr = class_wrapper.as_raw();
    println!("write cache: {:p}", ptr);
    cache.insert(class_name.to_string(), class_wrapper);

    (env, class)
}
