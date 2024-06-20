use jni::objects::*;
use jni::JNIEnv;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

static CLASS_CACHE: Lazy<RwLock<HashMap<String, JClass<'static>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

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
    // println!("{:p}", *j_class);
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
    {
        let cache = CLASS_CACHE.read().unwrap();
        // first try from cache
        let class = cache.get(class_name);
        if class.is_some() {
            println!("hit cache");
            let class = class.unwrap();
            println!("read cache: {:p}", class.as_raw());
            return (env, unsafe { JClass::from_raw(class.as_raw()) });
        } else {
            println!("not hit cache");
        }
    }
    let mut cache = CLASS_CACHE.write().unwrap();

    // or find from env
    let class = env.find_class(class_name).expect("class not found");
    let class_wrapper = unsafe { JClass::from_raw(class.as_raw()) };
    println!("write cache: {:p}", class_wrapper.as_raw());
    cache.insert(class_name.to_string(), class_wrapper);

    (env, class)
}
