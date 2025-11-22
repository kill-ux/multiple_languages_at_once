use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

#[unsafe(no_mangle)]
pub extern "system" fn Java_Adder_add(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_Adder_print(
    _env: JNIEnv,
    _class: JClass
) {
    println!("Hello World");
}