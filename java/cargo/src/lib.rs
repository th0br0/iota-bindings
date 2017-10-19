extern crate jni;

extern crate iota_bindings_shared;
extern crate iota_trytes as trytes;
extern crate iota_mam as mam;
extern crate iota_merkle as merkle;
extern crate iota_curl_cpu as curl_cpu;
extern crate iota_curl as curl;

mod util;

use std::boxed::Box;

use jni::JNIEnv;
use jni::objects::*;
use jni::sys::*;
use iota_bindings_shared::*;

/*
JNIEXPORT jlong JNICALL Java_rs_iota_jni_MerkleBranch_nativeLength
(JNIEnv *, jclass, jlong);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleBranch_nativeLength(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    let branch: &merkle::MerkleBranch = unsafe { std::mem::transmute(ptr) };
    merkle::len(branch) as i64
}

/*
JNIEXPORT void JNICALL Java_rs_iota_jni_MerkleBranch_nativeDrop
(JNIEnv *, jclass, jlong);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleBranch_nativeDrop(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe { Box::from_raw(ptr as *mut merkle::MerkleBranch) };
}

/*
JNIEXPORT jintArray JNICALL Java_rs_iota_jni_MerkleBranch_nativeSiblings
(JNIEnv *, jclass, jlong);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleBranch_nativeSiblings(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jintArray {
    let branch: &merkle::MerkleBranch = unsafe { std::mem::transmute(ptr) };

    let len = merkle::len(branch) * trytes::HASH_LENGTH;
    let mut out_trits: Vec<trytes::Trit> = vec![0; len];
    merkle::write_branch(&branch, len - trytes::HASH_LENGTH, &mut out_trits);

    util::trits_to_jintArray(&env, &out_trits)
}

/*
JNIEXPORT jlong JNICALL Java_rs_iota_jni_MerkleTree_nativeCreate
(JNIEnv *, jclass, jintArray, jint, jint, jint);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleTree_nativeCreate(
    env: JNIEnv,
    _class: JClass,
    seed: jintArray,
    index: jint,
    count: jint,
    security: jint,
) -> jlong {

    let seed = util::jintArray_to_trits(&env, seed);

    let mut c1 = curl_cpu::CpuCurl::<trytes::Trit>::default();
    let mut c2 = curl_cpu::CpuCurl::<trytes::Trit>::default();
    let mut c3 = curl_cpu::CpuCurl::<trytes::Trit>::default();

    let boxed = Box::new(merkle::create(
        &seed,
        index as isize,
        count as usize,
        security as usize,
        &mut c1,
        &mut c2,
        &mut c3,
    ));

    Box::into_raw(boxed) as jlong
}

/*
JNIEXPORT jlong JNICALL Java_rs_iota_jni_MerkleTree_nativeDepth
(JNIEnv *, jclass, jlong);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleTree_nativeDepth(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    let tree: &merkle::MerkleTree = unsafe { std::mem::transmute(ptr) };
    merkle::depth(tree) as i64
}

/*
JNIEXPORT jlong JNICALL Java_rs_iota_jni_MerkleTree_nativeSize
(JNIEnv *, jclass, jlong);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleTree_nativeSize(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    let tree: &merkle::MerkleTree = unsafe { std::mem::transmute(ptr) };
    merkle::size(tree) as i64
}
/*
JNIEXPORT void JNICALL Java_rs_iota_jni_MerkleTree_nativeDrop
(JNIEnv *, jclass, jlong);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleTree_nativeDrop(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe { Box::from_raw(ptr as *mut merkle::MerkleTree) };
}

/*
JNIEXPORT jlong JNICALL Java_rs_iota_jni_MerkleTree_nativeBranch
(JNIEnv *, jclass, jlong, jint);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleTree_nativeBranch(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    index: jint,
) -> jlong {
    let tree: &merkle::MerkleTree = unsafe { std::mem::transmute(ptr) };
    let branch = Box::new(merkle::branch(tree, index as usize));

    Box::into_raw(branch) as jlong
}

/*
JNIEXPORT jintArray JNICALL Java_rs_iota_jni_MerkleTree_nativeSlice
(JNIEnv *, jclass, jlong);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MerkleTree_nativeSlice(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jintArray {
    let tree: &merkle::MerkleTree = unsafe { std::mem::transmute(ptr) };
    let mut tmp = vec![0; trytes::HASH_LENGTH];
    merkle::slice(tree, &mut tmp);

    util::trits_to_jintArray(&env, &tmp)
}

/*
JNIEXPORT jintArray JNICALL Java_rs_iota_jni_MAM_nativeEncode
  (JNIEnv *, jclass, jintArray, jintArray, jintArray, jintArray, jintArray, jintArray, jint, jint, jint);
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MAM_nativeEncode(
    env: JNIEnv,
    _class: JClass,
    seedArr: jintArray,
    messageArr: jintArray,
    sidekeyArr: jintArray,
    rootArr: jintArray,
    siblingsArr: jintArray,
    nextRootArr: jintArray,
    start: jint,
    index: jint,
    security: jint,
) -> jintArray {

    let seed = util::jintArray_to_trits(&env, seedArr);
    let message = util::jintArray_to_trits(&env, messageArr);
    let sidekey = util::jintArray_to_trits(&env, sidekeyArr);
    let root = util::jintArray_to_trits(&env, rootArr);
    let siblings = util::jintArray_to_trits(&env, siblingsArr);
    let nextRoot = util::jintArray_to_trits(&env, nextRootArr);

    let mut c1 = curl_cpu::CpuCurl::<trytes::Trit>::default();
    let mut c2 = curl_cpu::CpuCurl::<trytes::Trit>::default();
    let mut cb = curl_cpu::CpuCurl::<trytes::BCTrit>::default();

    let mut out: Vec<trytes::Trit> = vec![
        0;
        trytes::num::round_third(mam::min_length(
        message.len(),
        siblings.len(),
        index as usize,
        security as usize,
    ) as i64) as usize
    ];

    mam::create::<
        curl_cpu::CpuCurl<trytes::Trit>,
        curl_cpu::CpuCurl<trytes::BCTrit>,
        curl_cpu::CpuHam,
    >(
        &seed,
        &message,
        &sidekey,
        &root,
        &siblings,
        &nextRoot,
        start as isize,
        index as usize,
        security as u8,
        &mut out,
        &mut c1,
        &mut c2,
        &mut cb,
    );

    return util::trits_to_jintArray(&env, &out);
}

/*
JNIEXPORT jintArray JNICALL Java_rs_iota_jni_MAM_nativeDecode
  (JNIEnv *, jclass, jintArray, jintArray, jintArray);
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_rs_iota_jni_MAM_nativeDecode(
    env: JNIEnv,
    _class: JClass,
    payloadArr: jintArray,
    sidekeyArr: jintArray,
    rootArr: jintArray,
) -> jobject {
    let mut payload = util::jintArray_to_trits(&env, payloadArr);
    let sidekey = util::jintArray_to_trits(&env, sidekeyArr);
    let root = util::jintArray_to_trits(&env, rootArr);

    let mut c1 = curl_cpu::CpuCurl::<trytes::Trit>::default();

    let result = mam::parse(&mut payload, &sidekey, &root, &mut c1);

    match result {
        Ok(r) => {
            let cls = env.find_class("rs/iota/jni/MAM$DecodeResult").unwrap();

            let message = util::trits_to_jintArray(&env, &payload[r.0 + trytes::HASH_LENGTH..r.1]);
            let nextRoot = util::trits_to_jintArray(&env, &payload[r.0..r.0 + trytes::HASH_LENGTH]);

            match env.new_object(
                cls,
                "([I[I)V",
                &[
                    JValue::from(message as jlong),
                    JValue::from(nextRoot as jlong),
                ],
            ) {
                Ok(jobj) => jobj.into_inner(),
                Err(what) => {
                    env.throw_new("java/lang/Exception", format!("Could not create DecodeResult: {:?}", what))
                        .unwrap();

                    std::ptr::null_mut::<_jobject>()
                }
            }
        }

        Err(what) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Invalid arguments to MAM::decode. Error: {:?}", what),
            ).unwrap();

            std::ptr::null_mut::<_jobject>()
        }
    }


}
