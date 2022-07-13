use crate::easycodec::*;
use crate::sim_context;
use crate::vec_box;
use base64::decode;
use base64::encode;
use sim_context::*;
use std::mem;
use vec_box::VecBox;

#[no_mangle]
pub extern "C" fn init_contract() {
    sim_context::log("init_contract");
}

#[no_mangle]
pub extern "C" fn upgrade() {
    sim_context::log("upgrade");
}

#[no_mangle]
pub extern "C" fn empty_method() {
    // sim_context::log("empty_method");
}

#[no_mangle]
pub extern "C" fn increase() {
    sim_context::log("invoke increase");
    let ctx = &mut sim_context::get_sim_context();

    let mut key = ctx.arg_default_blank("key");
    if key.len() == 0 {
        key = "counter1".to_string();
    }
    let r = ctx.get_state("count", &key);
    if r.is_err() {
        ctx.error("get state error.");
        return;
    }
    let count = r.unwrap();
    let counti32 = i32_from(count.as_slice());
    let count_new_i32 = counti32 + 1;
    let count_new = count_new_i32.to_le_bytes().to_vec();
    let is_ok = ctx.put_state("count", &key, count_new.as_slice());
    if is_ok == sim_context::SUCCESS_CODE {
        let s = format!("++ stone success count={}", count_new_i32.to_string());
        ctx.log(&s);
        let _ = ctx.ok(s.as_bytes());
    } else {
        let s = format!("++ stone fail old count={}", counti32.to_string());
        ctx.log(&s);
        let _ = ctx.error(&key);
    }
}

#[no_mangle]
pub extern "C" fn query() {
    sim_context::log("invoke query");
    let ctx = &mut sim_context::get_sim_context();

    let mut key = ctx.arg_default_blank("key");
    if key.len() == 0 {
        key = "counter1".to_string();
    }

    let r = ctx.get_state("count", &key);

    if r.is_err() {
        ctx.error("get state error.");
        return;
    }
    let count = r.unwrap();
    let count: i32 = i32_from(count.as_slice());

    let log_str = format!("query count {}", count);
    let r = count.to_string();

    ctx.log(&log_str);
    ctx.ok(r.as_bytes());
}

#[no_mangle]
pub extern "C" fn call_contract_self() {
    sim_context::log("==================call_contract_self start==================");
    let ctx = &mut sim_context::get_sim_context();
    let contract_name = ctx.arg_default_blank("contractName");

    let mut ec = EasyCodec::new();
    ec.add_string("contractName", contract_name.as_str());

    let method = "call_contract_self";
    let _ = ctx.call_contract(&contract_name, &method, ec);
    sim_context::log("==================call_contract_self done==================");
}

#[no_mangle]
pub extern "C" fn call_contract_test() {
    sim_context::log("==================call_contract_test start==================");
    let ctx = &mut sim_context::get_sim_context();
    let contract_name = ctx.arg_default_blank("contractName");
    let method = ctx.arg_default_blank("method");
    let count = ctx.arg_default_blank("count");

    let mut i = 1;
    if count.len() > 0 {
        i = count.parse().unwrap();
    }
    for _ in 0..i {
        let mut ec = EasyCodec::new();
        ec.add_string("key", "key01");

        let r = ctx.call_contract(&contract_name, &method, ec);
        if r.is_err() {
            ctx.error("call contract error.");
            return;
        }

        let fact_vec = r.unwrap();
        if fact_vec.len() == 0 {
            ctx.log("save done.None");
            ctx.ok("".as_bytes());
            return;
        }
        let fact_str = std::str::from_utf8(&fact_vec).unwrap();
        let msg = format!("[call_contract_test] call contract result: {:?}", fact_str);
        sim_context::log(&msg);
    }
    sim_context::log("==================call_contract_test done==================");
}

pub fn i32_from(amount: &[u8]) -> i32 {
    if amount.len() == 4 {
        // receiver account money bytes into i32
        let b: i32 = i32::from_le_bytes([
            *amount.get(0).unwrap(),
            *amount.get(1).unwrap(),
            *amount.get(2).unwrap(),
            *amount.get(3).unwrap(),
        ]);
        return b;
    } else {
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn dump() {
    sim_context::log("invoke dump");
    let ctx = &mut sim_context::get_sim_context();
    ctx.log(&ctx.get_creator_org_id());
    ctx.log(&ctx.get_creator_role());
    ctx.log(&ctx.get_creator_pub_key());
    ctx.log(&ctx.get_sender_org_id());
    ctx.log(&ctx.get_sender_pub_key());
    ctx.log(&ctx.get_sender_role());
    ctx.log(&ctx.get_tx_id());
    ctx.log(&ctx.get_block_height().to_string());
    ctx.ok("ok".as_bytes());
    ctx.error("error");
}

#[no_mangle]
pub extern "C" fn for_dag() {
    sim_context::log("invoke for_dag");
    let ctx = &mut sim_context::get_sim_context();

    let tx_id = ctx.get_tx_id();
    let split = tx_id.split("a");
    for s in split {
        ctx.put_state("test", s, "value".as_bytes());
    }
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn calc_json() {
    let ctx = &mut sim_context::get_sim_context();

    ctx.log("[rtest] input func: calc_json");
    let func_name = ctx.arg_default_blank("func_name");
    let mut data1 = ctx.arg_default_blank("data1");
    let mut data2 = ctx.arg_default_blank("data2");

    ctx.log(&format!("[rtest] calc_json[func_name]: {:?}", func_name));
    ctx.log(&format!("[rtest] calc_json[data1]: {:?}", data1));
    ctx.log(&format!("[rtest] calc_json[data2]: {:?}", data2));

    if data2.len() == 0 {
        data2 = "0".to_string();
    }
    if data1.len() == 0 {
        data1 = "0".to_string();
    }
    let idata1 = data1.parse::<i32>().unwrap();
    let idata2 = data2.parse::<i32>().unwrap();
    ctx.log(&format!("[rtest] calc_json[idata1]: {}", idata1));
    ctx.log(&format!("[rtest] calc_json[idata2]: {}", idata2));

    let mut result_str: String = String::new();
    let mut result: i32 = 0;
    let mut status = false;

    if func_name == "add" {
        result = idata1 + idata2;
        result_str = result.to_string();
        status = true
    } else if func_name == "sub" {
        result = idata1 - idata2;
        result_str = result.to_string();
        status = true
    } else if func_name == "mul" {
        result = idata1 * idata2;
        result_str = result.to_string();
        status = true
    } else if func_name == "div" {
        result = idata1 / idata2;
        result_str = result.to_string();
        status = true
    } else if func_name == "set_data" {
        let data3 = ctx.arg_default_blank("data3");
        let data4 = ctx.arg_default_blank("data4");
        ctx.log(&format!("[rtest] calc_json[data3]: {:?}", data3));
        ctx.log(&format!("[rtest] calc_json[data4]: {:?}", data4));
        ctx.put_state("zitao", &data3, data4.as_bytes());
        ctx.log(&format!(
            "[rtest] calc_json[func_name] result: {:?}",
            result_str
        ));
    } else if func_name == "failure" {
        let data3 = ctx.arg_default_blank("data3");
        ctx.log(&format!("[rtest] calc_json[data3]: {:?}", data3));
        ctx.log(&format!(
            "[rtest] calc_json[func_name] failure set result: {:?}",
            data3
        ));
        ctx.put_state("zitao", &func_name, data3.as_bytes());
        ctx.error("zitao test error");
    } else if func_name == "delete" {
        let data3 = ctx.arg_default_blank("data3");
        ctx.log(&format!("[rtest] calc_json[data3]: {:?}", data3));
        ctx.log(&format!(
            "[rtest] calc_json[func_name] delete name: {:?}",
            data3
        ));
        ctx.delete_state("zitao", &data3);
    } else {
        ctx.log("[rtest] panic test");
        ctx.put_state("zitao", &func_name, "panic".as_bytes());
        panic!("zitao test panic !!!");
    }

    if status {
        ctx.put_state("zitao", &func_name, result_str.as_bytes());
        ctx.log(&format!(
            "[rtest] calc_json[func_name] result_str: {:?} result:{}",
            result_str, result
        ));
    }
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn get_calc() {
    let ctx = &mut sim_context::get_sim_context();

    ctx.log("[rtest] input func: get_json");
    let func_name = ctx.arg_default_blank("func_name");
    ctx.log(&format!("[rtest] get_calc[func_name]: {:?}", func_name));

    let r = ctx.get_state("zitao", &func_name);
    if r.is_err() {
        ctx.error("call contract error.");
        return;
    }

    let result = r.unwrap();
    let result = std::str::from_utf8(&result).unwrap();
    ctx.log(&format!("[rtest] get_calc[func_name] result: {:?}", result));

    ctx.ok(result.as_bytes());
}

#[no_mangle]
pub extern "C" fn call_self() {
    let ctx = &mut sim_context::get_sim_context();

    ctx.log("[rtest] input func: call_self");
    let r = ctx.get_state("zitao", "callnum");

    if r.is_err() {
        ctx.error("get state error.");
        return;
    }
    let callnum_vec = r.unwrap();
    if callnum_vec.len() == 0 {
        ctx.log("not found callnum value");
        return;
    }

    let callnum_str = std::str::from_utf8(&callnum_vec).unwrap();
    let icallnum = callnum_str.parse::<i32>().unwrap();
    ctx.log(&format!("[rtest] change calc_json[callnum]: {}", icallnum));

    let icallnum = icallnum - 1;
    ctx.put_state("zitao", "callnum", icallnum.to_string().as_bytes());
    if icallnum < 1 {
        ctx.log(&format!(
            "[rtest] call_self[callnum] result(end): {}",
            icallnum
        ));
        ctx.ok("finish call_self".as_bytes());
    } else {
        ctx.log(&format!(
            "[rtest] call_self[callnum] result(test): {}",
            icallnum
        ));
        call_self()
    }
}

#[no_mangle]
pub extern "C" fn loop_test() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: loop_test");
    let ctx = &mut sim_context::get_sim_context();

    let r = ctx.get_state("zitao", "loopnum");
    if r.is_err() {
        ctx.error("call contract error.");
        return;
    }
    let num = r.unwrap();
    if num.len() == 0 {
        ctx.log("not found zitao loopnum");
        return;
    }

    let num = std::str::from_utf8(&num).unwrap();
    let iloopnum: i32 = num.parse().unwrap();
    ctx.log(&format!("[rtest] change loop_test[loopnum]: {}", iloopnum));

    for i in 0..iloopnum {
        ctx.log(&format!("[rtest] change loop_test[i]:  {}", i));
        ctx.put_state("zitao", "loopnum", i.to_string().as_bytes());
    }

    sim_context::log("[rtest] finish loop_test========================================end");
}

#[no_mangle]
pub extern "C" fn set_store() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: set_store");
    let ctx = &mut sim_context::get_sim_context();

    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    let value = ctx.arg_default_blank("value");
    ctx.log(&format!("[rtest] change set_store[key]: {:?}", key));
    ctx.log(&format!("[rtest] change set_store[field]: {:?}", field));
    ctx.log(&format!("[rtest] change set_store[value]: {:?}", value));
    ctx.put_state(&key, &field, value.as_bytes());

    sim_context::log("[rtest] ========================================end");
    sim_context::log("[rtest] finish set_store");
}

#[no_mangle]
pub extern "C" fn set_store_no_log() {
    let ctx = &mut sim_context::get_sim_context();
    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    let value = ctx.arg_default_blank("value");

    ctx.put_state(&key, &field, value.as_bytes());
}

#[no_mangle]
pub extern "C" fn get_store() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: get_store");
    let ctx = &mut sim_context::get_sim_context();

    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    ctx.log(&format!("[rtest] change set_store[key]: {:?}", key));
    ctx.log(&format!("[rtest] change set_store[field]: {:?}", field));

    let r = ctx.get_state(&key, &field);
    if r.is_err() {
        ctx.error("call contract error.");
        return;
    }
    let value = r.unwrap();
    let value = std::str::from_utf8(&value).unwrap();
    ctx.log(&format!("[rtest] change set_store[value]: {:?}", value));
    sim_context::log("[rtest] ========================================end");

    ctx.ok(value.as_bytes());
}

#[no_mangle]
pub extern "C" fn get_store_no_log() {
    let ctx = &mut sim_context::get_sim_context();

    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");

    let r = ctx.get_state(&key, &field);
    if r.is_err() {
        ctx.error("call contract error.");
        return;
    }
    let value = r.unwrap();
    let value = std::str::from_utf8(&value).unwrap();

    ctx.ok(value.as_bytes());
}

#[no_mangle]
pub extern "C" fn delete_store() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: delete_store");
    let ctx = &mut sim_context::get_sim_context();

    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    ctx.log(&format!("[rtest] change set_store[key]: {:?}", key));
    ctx.log(&format!("[rtest] change set_store[field]: {:?}", field));
    ctx.delete_state(&key, &field);
    sim_context::log("[rtest] ========================================end");
    sim_context::log("[rtest] finish delete_store");
}

#[no_mangle]
pub extern "C" fn test_get_state() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: state_get");
    let ctx = &mut sim_context::get_sim_context();

    let func_name = ctx.arg_default_blank("func_name");
    let key = ctx.arg_default_blank("key");
    let name = ctx.arg_default_blank("name");
    ctx.log(&format!("[rtest] change state_get[func_name]: {:?}", func_name));
    ctx.log(&format!("[rtest] change state_get[key]: {:?}", key));
    ctx.log(&format!("[rtest] change state_get[name]: {:?}", name));

    let result_state: Result<Vec<u8>, result_code>;
    if func_name == "GetState" {
        result_state = ctx.get_state(&key, &name);
    } else if func_name == "GetStateFromKey" {
        result_state = ctx.get_state_from_key(&key);
    } else {
        ctx.error(&format!(
            "finish state_get failure: error para: {}",
            func_name
        ));
        return;
    }
    if result_state.is_err() {
        ctx.log(&format!("[rtest] state_get [{}] error", func_name));
        ctx.error(&format!("state_get [{}] error", func_name));
        return;
    }
    let data = result_state.unwrap();
    let result = String::from_utf8(data);
    let result_str = result.unwrap();
    sim_context::log("[rtest] ========================================end");
    ctx.ok(result_str.as_bytes());
}

#[no_mangle]
pub extern "C" fn call_memory() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: call_memory");
    let ctx = &mut sim_context::get_sim_context();

    let allocate_size = ctx.arg_default_blank("allocate_size");
    ctx.log(&format!(
        "[rtest] call_memory[allocate_size]: {:?}",
        allocate_size
    ));

    let size = allocate_size.parse::<i32>().unwrap();

    let mut str_val = "1".to_string();
    let mut i = size;
    while i > 0 {
        str_val.push_str("1");
        i -= 1;
    }
    ctx.log(&format!(
        "[rtest] call_memory str len={} len(m)={}k ",
        str_val.len(),
        str_val.len() / 1024,
    ));

    sim_context::log("[rtest] ========================================end");
    sim_context::log("[rtest] finish call_memory");
}

#[no_mangle]
pub extern "C" fn call_allocate() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: call_allocate");
    let ctx = &mut sim_context::get_sim_context();

    let allocate_size = ctx.arg_default_blank("allocate_size");
    ctx.log(&format!(
        "[rtest] call_allocate[allocate_size]: {:?}",
        allocate_size
    ));

    let size = allocate_size.parse::<usize>().unwrap();
    let mut vec: VecBox<u8> = VecBox::new(size);
    ctx.log(&format!(
        "[rtest] call_allocate vec len: {} len:{}k ptr: {}",
        vec.len(),
        vec.len() / 1024,
        vec.as_ptr() as u32
    ));

    sim_context::log("[rtest] ========================================end");
    sim_context::log("[rtest] finish call_allocate");
}

#[no_mangle]
pub extern "C" fn call_allocate_type2() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: call_allocate");
    let ctx = &mut sim_context::get_sim_context();

    let allocate_size = ctx.arg_default_blank("allocate_size");
    ctx.log(&format!(
        "[rtest] call_allocate[allocate_size]: {:?}",
        allocate_size
    ));

    let size = allocate_size.parse::<usize>().unwrap();

    let mut buffer: Vec<u8> = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, size);
    }
    ctx.log(&format!(
        "[rtest] call_allocate vec len: {} len:{}k ",
        size,
        size / 1024
    ));

    sim_context::log("[rtest] ========================================end");
    sim_context::log("[rtest] finish call_allocate");
}

#[no_mangle]
pub extern "C" fn event_test_set_no_log() {
    let ctx = &mut sim_context::get_sim_context();

    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    let value = ctx.arg_default_blank("value");

    ctx.put_state(&key, &field, value.as_bytes());
    let datas = value.split("#");
    let mut evt = Vec::<String>::new();
    for data in datas {
        evt.push(data.to_string());
    }
    ctx.emit_event(&key, &evt);
    ctx.ok("finish event_test_set".as_bytes());
}

#[no_mangle]
pub extern "C" fn event_test_get_no_log() {
    let ctx = &mut sim_context::get_sim_context();

    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");

    let r = ctx.get_state(&key, &field);

    let datas = field.split("#");
    let mut evt = Vec::<String>::new();
    for data in datas {
        evt.push(data.to_string());
    }
    ctx.emit_event(&key, &evt);

    ctx.ok(r.unwrap().as_slice());
}

#[no_mangle]
pub extern "C" fn paillier_test_set() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: paillier_test_set");

    let ctx = sim_context::get_sim_context();
    let pubkey = ctx.arg_default_blank("pubkey");
    // let encode_pubkey = pubkey.into_bytes();
    // let decode_pubkey = decode(pubkey.into_bytes());

    let handletype = ctx.arg_default_blank("handletype");

    let para1 = ctx.arg_default_blank("para1");
    let decode_para1 = decode(para1.as_bytes()).unwrap();

    let para2 = ctx.arg_default_blank("para2");

    sim_context::log(&format!(
        "[rtest] change paillier_test_set[pubkey]:  {}",
        pubkey
    ));
    sim_context::log(&format!(
        "[rtest] change paillier_test_set[handletype]:  {}",
        handletype
    ));
    sim_context::log(&format!(
        "[rtest] change paillier_test_set[para1]:  {}",
        para1
    ));
    sim_context::log(&format!(
        "[rtest] change paillier_test_set[para2]:  {}",
        para2
    ));

    let test = ctx.get_paillier_sim_context();
    let r: Result<Vec<u8>, i32>;
    if handletype == "AddCiphertext" {
        let decode_para2 = decode(para2.as_bytes()).unwrap();
        r = test.add_ciphertext(pubkey.into_bytes(), decode_para1, decode_para2);
    } else if handletype == "AddPlaintext" {
        // let decode_para2: i64 = para2.parse().unwrap();
        r = test.add_plaintext(pubkey.into_bytes(), decode_para1, &para2);
    } else if handletype == "SubCiphertext" {
        let decode_para2 = decode(para2.as_bytes()).unwrap();
        r = test.sub_ciphertext(pubkey.into_bytes(), decode_para1, decode_para2);
    } else if handletype == "SubPlaintext" {
        // let decode_para2: i64 = para2.parse().unwrap();
        r = test.sub_plaintext(pubkey.into_bytes(), decode_para1, &para2);
    } else if handletype == "NumMul" {
        // let decode_para2: i64 = para2.parse().unwrap();
        r = test.num_mul(pubkey.into_bytes(), decode_para1, &para2);
    } else {
        ctx.error(&format!(
            "finish event_test_set failure: error para: {}",
            handletype
        ));
        return;
    }
    if r.is_err() {
        ctx.error("finish event_test_set failure");
        return;
    }

    let data = r.unwrap();
    let data_u8 = data.as_slice();
    // let data_r = String::from_utf8(data_u8.to_vec());
    // let data_str: String;
    // if data_r.is_err() {
    // data_str = "not utf8 char".to_string();
    let data_str = encode(data_u8);
    // } else {
    //     // data_str = data_r.unwrap();
    //     data_str = encode(data_r.unwrap())
    // }
    let put_code = ctx.put_state("paillier_test", &handletype, data_str.as_bytes());

    ctx.log(&format!(
        "[rtest] PutState: key=paillier_test_set ,name={}, value={} result={}",
        handletype, data_str, put_code
    ));
    ctx.log("[rtest] ========================================end");
    ctx.ok("finish event_test_set success".as_bytes());
}

#[no_mangle]
pub extern "C" fn paillier_test_get() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: paillier_test_get");

    let ctx = sim_context::get_sim_context();
    let handletype = ctx.arg_default_blank("handletype");
    let r = ctx.get_state("paillier_test", &handletype);
    if r.is_err() {
        sim_context::log("[rtest] paillier_test_get error");
        ctx.error("finish paillier_test_get failure");
        return;
    }
    let data = r.unwrap();

    let result = String::from_utf8(data);
    let result_str = result.unwrap();
    ctx.log(&format!(
        "[rtest] GetState: key=paillier_test_get ,name={}， value={} result={}",
        handletype, result_str, 0
    ));
    ctx.log("[rtest] ========================================end");

    ctx.ok(result_str.as_bytes());
}

#[no_mangle]
pub extern "C" fn bulletproofs_test_set() {
    sim_context::log("[bulletproofs] ========================================start");
    sim_context::log("[bulletproofs] bulletproofs_test_set");

    let ctx = sim_context::get_sim_context();
    let handle_type = ctx.arg_default_blank("handletype");
    let para1 = ctx.arg_default_blank("para1");
    let decode_para1 = decode(para1.as_bytes()).unwrap();
    let para2 = ctx.arg_default_blank("para2");
    // let decode_para2 = decode(para2.as_bytes()).unwrap();

    sim_context::log(&format!(
        "[bulletproofs] bulletproofs_test_set [handletype]: {}",
        handle_type
    ));

    sim_context::log(&format!(
        "[bulletproofs] bulletproofs_test_set [para1]: {}",
        para1
    ));

    sim_context::log(&format!(
        "[bulletproofs] bulletproofs_test_set [para2]: {}",
        para2
    ));

    let test = ctx.get_bulletproofs_sim_context();
    let result: Result<Vec<u8>, i32>;

    if handle_type == "PedersenAddNum" {
        // let decode_para2: u64 = para2.parse().unwrap();
        // let decode_para2 = para2.parse::<u64>().unwrap();
        result = test.pedersen_add_num(decode_para1, &para2)
    } else if handle_type == "PedersenAddCommitment" {
        let decode_para2 = decode(para2.as_bytes()).unwrap();
        result = test.pedersen_add_commitment(decode_para1, decode_para2)
    } else if handle_type == "PedersenSubNum" {
        // let decode_para2: u64 = para2.parse().unwrap();
        result = test.pedersen_sub_num(decode_para1, &para2)
    } else if handle_type == "PedersenSubCommitment" {
        let decode_para2 = decode(para2.as_bytes()).unwrap();
        result = test.pedersen_sub_commitment(decode_para1, decode_para2)
    } else if handle_type == "PedersenMulNum" {
        // let decode_para2: u64 = para2.parse().unwrap();
        result = test.pedersen_mul_num(decode_para1, &para2)
    } else if handle_type == "BulletproofsVerify" {
        let decode_para2 = decode(para2.as_bytes()).unwrap();
        result = test.verify(decode_para1, decode_para2)
    } else {
        ctx.error(&format!(
            "finish event_test_set failure: error para: {}",
            handle_type
        ));
        return;
    }

    if result.is_err() {
        ctx.error("finish event_test_set failure");
        return;
    }

    let data = result.unwrap();
    let data_u8 = data.as_slice();
    let data_str = encode(data_u8);
    let put_code = ctx.put_state("bulletproofs_test", &handle_type, data_str.as_bytes());

    ctx.log(&format!(
        "[bulletproofs] PutState: key=bulletproofs_test, name={}, value={}, result={}",
        handle_type, data_str, put_code
    ));
    ctx.log("[bulletproofs] ========================================end");
    // ctx.ok("finish event_test_set success".as_bytes());
    ctx.ok(data_str.as_bytes());
}

#[no_mangle]
pub extern "C" fn bulletproofs_test_get() {
    sim_context::log("[bulletproofs] ========================================end");
    sim_context::log("[bulletproofs] bulletproofs_test_get");

    let ctx = sim_context::get_sim_context();
    let handle_type = ctx.arg_default_blank("handletype");
    let result = ctx.get_state("bulletproofs_test", &handle_type);
    if result.is_err() {
        sim_context::log("[bulletproofs] bulletproofs_test_get error");
        ctx.error("finish bulletproofs_test_get failure");
        return;
    }
    let data = result.unwrap();
    let result = String::from_utf8(data);
    let result_str = result.unwrap();
    ctx.log(&format!(
        "[bulletproofs] GetState: key=bulletproofs_test_get, name={}, value={}, result={}",
        handle_type, result_str, 0
    ));

    ctx.log("[bulletproofs] ========================================end");

    if handle_type == "BulletproofsVerify" {
        ctx.ok(&*decode(result_str.as_bytes()).unwrap());
    } else {
        ctx.ok(result_str.as_bytes());
    }
}

#[no_mangle]
pub extern "C" fn kv_iterator_put_single_state() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: kv_iteartor_set");
    let ctx = &mut sim_context::get_sim_context();

    let func_name = ctx.arg_default_blank("func_name");
    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    let value = ctx.arg_default_blank("value");
    ctx.log(&format!("[rtest] change kv_iteartor_set[func_name]: {:?}", func_name));
    ctx.log(&format!("[rtest] change kv_iteartor_set[key]: {:?}", key));
    ctx.log(&format!("[rtest] change kv_iteartor_set[field]: {:?}", field));
    ctx.log(&format!("[rtest] change kv_iteartor_set[value]: {:?}", value));

    if func_name == "PutState" {
        ctx.put_state(&key, &field, value.as_bytes());
    } else if func_name == "PutStateFromKey" {
        ctx.put_state_from_key(&key, value.as_bytes());
    } else {
        ctx.error(&format!(
            "finish kv_iteartor_set failure: error para: {}",
            func_name
        ));
        return;
    }
    sim_context::log("[rtest] ========================================end");
    sim_context::log("[rtest] finish kv_iteartor_set");
}

#[no_mangle]
pub extern "C" fn kv_iterator_put_multi_state() {
    let ctx = sim_context::get_sim_context();
    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    let value = ctx.arg_default_blank("value");
    let count_str = ctx.arg_default_blank("count");
    let start_count_str = ctx.arg_default_blank("start_count");

    let count = count_str.parse::<i32>();
    if count.is_err() {
        let msg = format!("count is {:?} not int32 number.", count_str);
        ctx.log(&msg);
        ctx.error(&msg);
        return;
    }
    let start_count = start_count_str.parse::<i32>().unwrap();

    if key.len() == 0 || value.len() == 0 {
        let msg = format!("key or value is null");
        ctx.log(&msg);
        ctx.error(&msg);
        return;
    }
    for i in start_count..count.unwrap() {
        let field = format!("{}{}", field, i);
        ctx.put_state(&key, &field, value.as_bytes());
    }
}

#[no_mangle]
pub extern "C" fn kv_iterator_put_and_get_multi_state() {
    let ctx = sim_context::get_sim_context();
    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    let value = ctx.arg_default_blank("value");
    let count_str = ctx.arg_default_blank("count");
    let start_count_str = ctx.arg_default_blank("start_count");

    let count = count_str.parse::<i32>();
    if count.is_err() {
        let msg = format!("count is {:?} not int32 number.", count_str);
        ctx.log(&msg);
        ctx.error(&msg);
        return;
    }
    let start_count = start_count_str.parse::<i32>().unwrap();

    if key.len() == 0 || value.len() == 0 {
        let msg = format!("key or value is null");
        ctx.log(&msg);
        ctx.error(&msg);
        return;
    }
    for i in start_count..count.unwrap() {
        let field = format!("{}{}", field, i);
        ctx.put_state(&key, &field, value.as_bytes());
    }
    kv_iterator_get();
}

#[no_mangle]
pub extern "C" fn kv_iterator_get() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: kv_iteartor_get");
    let ctx = &mut sim_context::get_sim_context();

    let func_name = ctx.arg_default_blank("func_name");
    let start_key = ctx.arg_default_blank("start_key");
    let start_field = ctx.arg_default_blank("start_field");
    let limit_key = ctx.arg_default_blank("limit_key");
    let limit_field = ctx.arg_default_blank("limit_field");
    ctx.log(&format!("[rtest] change kv_iteartor_set[func_name]: {:?}", func_name));
    ctx.log(&format!("[rtest] change kv_iteartor_set[start_key]: {:?}", start_key));
    ctx.log(&format!("[rtest] change kv_iteartor_set[start_field]: {:?}", start_field));
    ctx.log(&format!("[rtest] change kv_iteartor_set[limit_key]: {:?}", limit_key));
    ctx.log(&format!("[rtest] change kv_iteartor_set[limit_field]: {:?}", limit_field));

    // let r: Result<Box<dyn ResultSet>, result_code>;
    let mut result = "".to_string();
    let r;
    if func_name == "NewIterator" {
        r = ctx.new_iterator(&start_key, &limit_key);
    } else if func_name == "NewIteratorWithField" {
        r = ctx.new_iterator_with_field(&start_key, &start_field, &limit_field);
    } else if func_name == "NewIteratorPrefixWithKeyField" {
        r = ctx.new_iterator_prefix_with_key_field(&start_key, &start_field);
    } else if func_name == "NewIteratorPrefixWithKey" {
        r = ctx.new_iterator_prefix_with_key(&start_key);
    } else {
        ctx.error(&format!(
            "finish kv_iteartor_get failure: error para: {}",
            func_name
        ));
        return;
    }
    if r.is_err() {
        ctx.log(&format!("[rtest] kv_iteartor_get [{}] error", func_name));
        ctx.error(&format!("kv_iteartor_get [{}] error", func_name));
        return;
    }
    let rs = r.unwrap();
    while rs.has_next() {
        let row = rs.next_row();
        let row = row.unwrap();

        let k = row.get_string("key").unwrap();
        let n = row.get_string("field").unwrap();
        let value = row.get_bytes("value");
        // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
        let v = String::from_utf8(value.unwrap()).unwrap();
        ctx.log(&format!("[rtest] new_iterator_from_key k=[{}] v=[{}]", k, v));
        result.push_str("{");
        // let json_str = format!("\"key\":\"{k:?}\",\"field\":\"{n:?}\",\"value\":\"{v:?}\"", k=k, n=n, v=v);
        let json_str = format!("\"key\":\"{}\",\"field\":\"{}\",\"value\":\"{}\"", k, n, v);
        ctx.log(&format!("[rtest] json_str=[{}] ", json_str));
        result.push_str(&json_str);
        result.push_str("}");
        // json_str.close();
        result.push_str(",");
    }
    rs.close();
    if result.len() > 0 {
        result.remove(result.len() - 1);
    }
    ctx.log(&format!("[rtest] result=[{}] ", result));
    ctx.log("[rtest] sql_handle_query success.");
    ctx.ok(format!("[{}]", result).as_bytes());
}

#[no_mangle]
pub extern "C" fn kv_iterator_get_and_set_combined_state() {
    sim_context::log("[rtest] ========================================start");
    sim_context::log("[rtest] input func: kv_iteartor_get");
    let ctx = &mut sim_context::get_sim_context();

    let func_name = ctx.arg_default_blank("func_name");
    let start_key = ctx.arg_default_blank("start_key");
    let start_field = ctx.arg_default_blank("start_field");
    let limit_key = ctx.arg_default_blank("limit_key");
    let limit_field = ctx.arg_default_blank("limit_field");
    ctx.log(&format!("[rtest] change kv_iteartor_set[func_name]: {:?}", func_name));
    ctx.log(&format!("[rtest] change kv_iteartor_set[start_key]: {:?}", start_key));
    ctx.log(&format!("[rtest] change kv_iteartor_set[start_field]: {:?}", start_field));
    ctx.log(&format!("[rtest] change kv_iteartor_set[limit_key]: {:?}", limit_key));
    ctx.log(&format!("[rtest] change kv_iteartor_set[limit_field]: {:?}", limit_field));

    // let r: Result<Box<dyn ResultSet>, result_code>;
    let mut result = "".to_string();
    let r;
    if func_name == "NewIterator" {
        r = ctx.new_iterator(&start_key, &limit_key);
    } else if func_name == "NewIteratorWithField" {
        r = ctx.new_iterator_with_field(&start_key, &start_field, &limit_field);
    } else if func_name == "NewIteratorPrefixWithKeyField" {
        r = ctx.new_iterator_prefix_with_key_field(&start_key, &start_field);
    } else if func_name == "NewIteratorPrefixWithKey" {
        r = ctx.new_iterator_prefix_with_key(&start_key);
    } else {
        ctx.error(&format!(
            "finish kv_iteartor_get failure: error para: {}",
            func_name
        ));
        return;
    }
    if r.is_err() {
        ctx.log(&format!("[rtest] kv_iteartor_get [{}] error", func_name));
        ctx.error(&format!("kv_iteartor_get [{}] error", func_name));
        return;
    }
    let rs = r.unwrap();
    while rs.has_next() {
        let row = rs.next_row();
        let row = row.unwrap();

        let k = row.get_string("key").unwrap();
        let n = row.get_string("field").unwrap();
        let value = row.get_bytes("value");
        // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
        let v = String::from_utf8(value.unwrap()).unwrap();
        ctx.log(&format!("[rtest] new_iterator_from_key k=[{}] v=[{}]", k, v));
        result.push_str("{");
        // let json_str = format!("\"key\":\"{k:?}\",\"field\":\"{n:?}\",\"value\":\"{v:?}\"", k=k, n=n, v=v);
        let json_str = format!("\"key\":\"{}\",\"field\":\"{}\",\"value\":\"{}\"", k, n, v);
        ctx.log(&format!("[rtest] json_str=[{}] ", json_str));
        result.push_str(&json_str);
        result.push_str("}");
        // json_str.close();
        result.push_str(",");
    }
    rs.close();
    if result.len() > 0 {
        result.remove(result.len() - 1);
    }
    let mut all_field = start_field;
    all_field.push_str(&limit_field);
    ctx.put_state(&start_key, &all_field, result.as_bytes());
}

#[no_mangle]
pub extern "C" fn kv_iterator_state_count() {
    let ctx = sim_context::get_sim_context();
    let key = ctx.arg_default_blank("key");
    let field = ctx.arg_default_blank("field");
    let count_str = ctx.arg_default_blank("count");
    let start_count_str = ctx.arg_default_blank("start_count");

    let count = count_str.parse::<i32>();
    if count.is_err() {
        let msg = format!("count is {:?} not int32 number.", count_str);
        ctx.log(&msg);
        ctx.error(&msg);
        return;
    }
    let start_count = start_count_str.parse::<i32>().unwrap();
    if key.len() == 0 {
        let msg = format!("key is null");
        ctx.log(&msg);
        ctx.error(&msg);
        return;
    }
    let mut total: i32 = 0;
    for i in start_count..count.unwrap() {
        let field = format!("{}{}", field, i);
        let r = ctx.get_state(&key, &field);
        if r.is_ok() {
            total += 1;
        }
    }
    ctx.log(&format!("iterator count={}", total));
    ctx.ok(format!("{}", total).as_bytes());
}
