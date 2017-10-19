
use trytes::Trit;
use jni::JNIEnv;
use jni::sys::jintArray;

#[allow(non_snake_case)]
pub fn jintArray_to_trits(env: &JNIEnv, array: jintArray) -> Vec<Trit> {
    let length =env.get_array_length(array).unwrap();
    let mut tmp: Vec<i32> = vec![0; length as usize];
    env.get_int_array_region(array, 0, &mut tmp).unwrap();

    tmp.iter().map(|t: &i32| *t as Trit).collect()
}

#[allow(non_snake_case)]
pub fn trits_to_jintArray(env: &JNIEnv, trits: &[Trit]) -> jintArray {
    let len = trits.len();
    let array = env.new_int_array(len as i32).unwrap();

    let tmp : Vec<i32> = trits.iter().map(|t: &Trit| *t as i32).collect();

    env.set_int_array_region(array, 0, &tmp).unwrap();

    array
}
