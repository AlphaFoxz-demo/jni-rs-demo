use jni::objects::*;
use jni::JNIEnv;

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
