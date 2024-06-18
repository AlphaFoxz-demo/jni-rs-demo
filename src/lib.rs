use jni::objects::*;
use jni::JNIEnv;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

static CLASS_CACHE: Lazy<RwLock<HashMap<String, JClass<'static>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[no_mangle]
pub extern "C" fn Java_com_github_alphafoxz_oneboot_core_NativeLib_update(
    mut env: JNIEnv,
    _class: JClass,
    complex_object: JObject,
) -> () {
    let j_class = env
        .find_class("com/github/alphafoxz/oneboot/core/ComplexObject")
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
pub unsafe extern "C" fn Java_com_github_alphafoxz_oneboot_core_NativeLib_alloc(
    mut env: JNIEnv<'static>,
    _: JClass,
) -> JObject<'static> {
    let j_class = env
        .find_class("com/github/alphafoxz/oneboot/core/ComplexObject")
        .expect("class not found");
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
pub unsafe extern "C" fn Java_com_github_alphafoxz_oneboot_core_NativeLib_allocWithCache<'a>(
    env: JNIEnv<'a>,
    _: JClass,
) -> JObject<'a> {
    let (mut env, j_class) =
        get_cached_class(env, "com/github/alphafoxz/oneboot/core/ComplexObject");
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

// 一个辅助函数来查找并缓存JClass
fn get_cached_class<'a>(mut env: JNIEnv<'a>, class_name: &str) -> (JNIEnv<'a>, JClass<'a>) {
    {
        let map = CLASS_CACHE.read().unwrap();
        // 首先尝试从缓存中获取JClass
        let class = map.get(class_name);
        if class.is_some() {
            return unsafe { (env, JClass::from_raw(***class.unwrap())) };
        }
    }
    let mut map = CLASS_CACHE.write().unwrap();

    // 如果缓存中没有，就通过find_class查找，并添加到缓存中
    let class = env.find_class(class_name).unwrap();
    let class_wrapper = unsafe { JClass::from_raw(**class) };
    map.insert(class_name.to_string(), class_wrapper);

    (env, class)
}
