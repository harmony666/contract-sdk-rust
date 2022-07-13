/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context;
use crate::sim_context_rs;
use sim_context::*;
use sim_context_rs::*;

#[no_mangle]
pub extern "C" fn init_contract() {
    sim_context::log("init_contract start");
    let ctx = &mut sim_context::get_sim_context();
    sim_context::log("init_contract success");
    ctx.ok("init_contract ok".as_bytes());
    sim_context::log("init_contract end");
}

#[no_mangle]
pub extern "C" fn upgrade() {
    sim_context::log("upgrade start");
    let ctx = &mut sim_context::get_sim_context();
    ctx.ok("upgrade ok".as_bytes());
    sim_context::log("upgrade success");
}

#[no_mangle]
pub extern "C" fn test_kv_iterator() {
    let ctx = &mut sim_context::get_sim_context();
    let key = ctx.arg_as_utf8_str("key");
    let limit = ctx.arg_as_utf8_str("limit");
    if key.len() > 0 {
        let r = ctx.new_iterator_with_field("key", &key, &limit);
        if r.is_err() {
            ctx.log("test_kv_iterator new_iterator error");
            ctx.error("test_kv_iterator new_iterator error");
            return;
        }
        let rs = r.unwrap();
        while rs.has_next() {
            let row = rs.next_row();
            let row = row.unwrap();
            let k = row.get_string("key").unwrap();
            let val = row.get_bytes("value");
            // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
            let v = String::from_utf8(val.unwrap()).unwrap();
            ctx.log(&format!("iterator k=[{}] v=[{}]", k, v));
        }
        rs.close();
    }
    let mut count: i32 = 0;
    {
        let key = "aabcd1234";
        let limit = "aabcd1248";
        ctx.log(&format!("test_kv_iterator key={}, limit={}", key, limit));
        let r = ctx.new_iterator_with_field("key", &key, &limit);
        if r.is_err() {
            ctx.log("test_kv_iterator new_iterator error");
            ctx.error("test_kv_iterator new_iterator error");
            return;
        }
        let rs = r.unwrap();
        while rs.has_next() {
            let row = rs.next_row();
            let row = row.unwrap();
            let k = row.get_string("key").unwrap();
            let val = row.get_bytes("value");
            // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
            let v = String::from_utf8(val.unwrap()).unwrap();
            ctx.log(&format!("test_kv_iterator k=[{}] v=[{}]", k, v));
            count += 1;
        }
        rs.close();
    }
    {
        let key = "key1234";
        let limit = "key1236";
        ctx.log(&format!(
            "new_iterator_from_key key={}, limit={}",
            key, limit
        ));
        let r = ctx.new_iterator(&key, &limit);
        if r.is_err() {
            ctx.log("new_iterator_from_key new_iterator error");
            ctx.error("new_iterator_from_key new_iterator error");
            return;
        }
        let rs = r.unwrap();
        while rs.has_next() {
            let row = rs.next_row();
            let row = row.unwrap();
            let k = row.get_string("key").unwrap();
            let val = row.get_bytes("value");
            // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
            let v = String::from_utf8(val.unwrap()).unwrap();
            ctx.log(&format!("new_iterator_from_key k=[{}] v=[{}]", k, v));
            count += 1;
        }
        rs.close();
    }
    {
        let key = "aabcd1241";
        let limit = "aabcd1271";
        ctx.log(&format!("test_kv_iterator key={}, limit={}", key, limit));
        let r = ctx.new_iterator_with_field("key", &key, &limit);
        if r.is_err() {
            ctx.log("test_kv_iterator new_iterator error");
            ctx.error("test_kv_iterator new_iterator error");
            return;
        }
        let rs = r.unwrap();
        while rs.has_next() {
            let row = rs.next_row();
            let row = row.unwrap();
            let k = row.get_string("key").unwrap();
            let val = row.get_bytes("value");
            // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
            let v = String::from_utf8(val.unwrap()).unwrap();
            ctx.log(&format!("test_kv_iterator k=[{}] v=[{}]", k, v));
            count += 1;
        }
        rs.close();
    }
    {
        let key = "aabcd1241";
        let limit = "aabcf1261";
        ctx.log(&format!("test_kv_iterator key={}, limit={}", key, limit));
        let r = ctx.new_iterator_with_field("key", &key, &limit);
        if r.is_err() {
            ctx.log("test_kv_iterator new_iterator error");
            ctx.error("test_kv_iterator new_iterator error");
            return;
        }
        let rs = r.unwrap();
        while rs.has_next() {
            let row = rs.next_row();
            let row = row.unwrap();
            let k = row.get_string("key").unwrap();
            let val = row.get_bytes("value");
            // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
            let v = String::from_utf8(val.unwrap()).unwrap();
            ctx.log(&format!("test_kv_iterator k=[{}] v=[{}]", k, v));
            count += 1;
        }
        rs.close();
    }
    if count == 15 {
        ctx.ok(count.to_string().as_bytes());
    } else {
        ctx.error(&format!("test_kv_iterator error count = {}", count));
    }
}

#[no_mangle]
pub extern "C" fn test_put_state() {
    let ctx = &mut sim_context::get_sim_context();
    ctx.log("test_put_state start");
    let key = ctx.arg_as_utf8_str("key");
    let value = ctx.arg_as_utf8_str("value");
    if key.len() > 0 {
        ctx.put_state("key", &key, value.as_bytes());
        ctx.log(&format!(
            "test_put_state finish key=[{}] value=[{}]",
            key, value
        ));
        return;
    }

    ctx.put_state("key", "aabcd1234", "aabcd1234".as_bytes());
    ctx.put_state("key", "aabcd1235", "aabcd1235".as_bytes());
    ctx.put_state("key", "aabcd1236", "aabcd1236".as_bytes());
    ctx.put_state("key", "aabcd1237", "aabcd1237".as_bytes());
    ctx.put_state("key", "aabcd1238", "aabcd1238".as_bytes());

    ctx.put_state("key", "aabcd1241", "aabcd1241".as_bytes());
    ctx.put_state("key", "aabcd1251", "aabcd1251".as_bytes());
    ctx.put_state("key", "aabcd1261", "aabcd1261".as_bytes());

    ctx.put_state("key", "aabcd1241", "aabcd1241".as_bytes());
    ctx.put_state("key", "aabce1251", "aabce1251".as_bytes());
    ctx.put_state("key", "aabcf1261", "aabcf1261".as_bytes());

    ctx.put_state_from_key("key1234", "key1234".as_bytes());
    ctx.put_state_from_key("key1235", "key1235".as_bytes());
    ctx.put_state_from_key("key1236", "key1236".as_bytes());

    ctx.ok("test_put_state ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn test_put_pre_state() {
    // 获取上下文
    let ctx = &mut sim_context::get_sim_context();
    ctx.log("test_put_pre_state start");
    let key = ctx.arg_as_utf8_str("key");
    let field = ctx.arg_as_utf8_str("field");
    let value = ctx.arg_as_utf8_str("value");
    if key.len() > 0 {
        ctx.put_state(&key, &field, value.as_bytes());
        ctx.log(&format!(
            "test_put_pre_state finish key=[{}] field=[{}] value=[{}]",
            key, field, value
        ));
        return;
    }
    ctx.put_state("123", "pre1", "123".as_bytes());
    ctx.put_state("123", "pre11", "123".as_bytes());
    ctx.put_state("123", "pre9", "123".as_bytes());
    ctx.put_state("123", "pre91", "123".as_bytes());
    ctx.put_state("123", "prea", "123".as_bytes());
    ctx.put_state("123", "prea1", "123".as_bytes());
    ctx.put_state("123", "prez", "123".as_bytes());
    ctx.put_state("123", "prez1", "123".as_bytes());
    ctx.put_state("123", "pre.", "123".as_bytes());
    ctx.put_state("123", "pre.1", "123".as_bytes());
    ctx.put_state("123", "pre-", "123".as_bytes());
    ctx.put_state("123", "pre-1", "123".as_bytes());
    ctx.put_state("123", "pre_", "123".as_bytes());
    ctx.put_state("123", "pre_1", "123".as_bytes());

    ctx.put_state_from_key("pre1", "123".as_bytes());
    ctx.put_state_from_key("pre11", "123".as_bytes());
    ctx.put_state_from_key("pre9", "123".as_bytes());
    ctx.put_state_from_key("pre91", "123".as_bytes());
    ctx.put_state_from_key("prea", "123".as_bytes());
    ctx.put_state_from_key("prea1", "123".as_bytes());
    ctx.put_state_from_key("prez", "123".as_bytes());
    ctx.put_state_from_key("prez1", "123".as_bytes());
    ctx.put_state_from_key("pre.", "123".as_bytes());
    ctx.put_state_from_key("pre.1", "123".as_bytes());
    ctx.put_state_from_key("pre-", "123".as_bytes());
    ctx.put_state_from_key("pre-1", "123".as_bytes());
    ctx.put_state_from_key("pre_", "123".as_bytes());
    ctx.put_state_from_key("pre_1", "123".as_bytes());
}

fn parse_rs_kv_iterator(
    ctx: &mut dyn SimContext,
    r: Result<Box<dyn ResultSet>, result_code>,
) -> i32 {
    if r.is_err() {
        return 0;
    }
    let mut count = 0;
    let rs = r.unwrap();
    while rs.has_next() {
        let row = rs.next_row();
        let row = row.unwrap();
        let k = row.get_string("key").unwrap();
        let val = row.get_bytes("value");
        // 注意，如果不是utf-8标准字符串，此处会报错。此处转为字符串仅为打印测试使用
        let v = String::from_utf8(val.unwrap()).unwrap();
        ctx.log(&format!("iterator k=[{}] v=[{}]", k, v));
        count += 1;
    }
    rs.close();

    count
}
#[no_mangle]
pub extern "C" fn test_iter_pre_field() {
    let ctx = &mut sim_context::get_sim_context();
    let mut count = 0;

    let rs = ctx.new_iterator_prefix_with_key_field("123", "pre1");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key_field("123", "pre9");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key_field("123", "prea");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key_field("123", "prez");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key_field("123", "pre.");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key_field("123", "pre-");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key_field("123", "pre_");
    count += parse_rs_kv_iterator(ctx, rs);

    ctx.ok(count.to_string().as_bytes());
}

#[no_mangle]
pub extern "C" fn test_iter_pre_key() {
    let ctx = &mut sim_context::get_sim_context();
    let mut count = 0;

    let rs = ctx.new_iterator_prefix_with_key("pre1");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key("pre9");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key("prea");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key("prez");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key("pre.");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key("pre-");
    count += parse_rs_kv_iterator(ctx, rs);
    let rs = ctx.new_iterator_prefix_with_key("pre_");
    count += parse_rs_kv_iterator(ctx, rs);

    ctx.ok(count.to_string().as_bytes());
}

// fact 保存
#[no_mangle]
pub extern "C" fn save() {
    // 获取上下文
    let ctx = &mut sim_context::get_sim_context();

    // 获取传入参数
    let file_hash = ctx.arg_as_utf8_str("file_hash");
    let file_name = ctx.arg_as_utf8_str("file_name");
    let time = ctx.arg_as_utf8_str("time");

    let mut ec = EasyCodec::new();
    ec.add_string("file_hash", file_hash.as_str());
    ec.add_string("file_name", file_name.as_str());
    ec.add_string("time", time.as_str());

    // 存储
    ctx.put_state("fact2", &file_hash, ec.marshal().as_slice());
    ctx.put_state("fact", &file_hash, ec.to_json().as_bytes());
}

// fact 查询
#[no_mangle]
pub extern "C" fn find_by_file_hash() {
    // 获取上下文
    let ctx = &mut sim_context::get_sim_context();

    // 获取传入参数
    let file_hash = ctx.arg_as_utf8_str("file_hash");

    // 校验参数
    if file_hash.len() == 0 {
        ctx.log("file_hash is null");
        ctx.ok("".as_bytes());
        return;
    }

    // 查询
    let r = ctx.get_state("fact", &file_hash);

    // 校验返回结果
    if r.is_err() {
        ctx.log("get_state fail");
        ctx.error("get_state fail");
        return;
    }
    let fact_vec = r.unwrap();
    if fact_vec.len() == 0 {
        ctx.log("None");
        ctx.ok("".as_bytes());
        return;
    }

    // 序列化
    let fact_str = std::str::from_utf8(&fact_vec).unwrap();

    // 打印日志
    ctx.log("get bytes data");
    ctx.log(fact_str);
    // 返回查询结果
    ctx.ok(fact_str.as_bytes());

    ctx.log("get marshal data");
    let fact_vec = ctx.get_state("fact2", &file_hash).unwrap();
    let data = EasyCodec::unmarshal(&fact_vec);
    ctx.log(&data.to_json());
}

// counter 递增
#[no_mangle]
pub extern "C" fn increase() {
    sim_context::log("invoke increase");
    let ctx = &mut sim_context::get_sim_context();

    let mut key = ctx.arg_as_utf8_str("key");
    if key.len() == 0 {
        key = "counter1".to_string();
    }
    let count = ctx.get_state("count", &key).unwrap();

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

// counter 查询
#[no_mangle]
pub extern "C" fn query() {
    sim_context::log("invoke query");
    let ctx = &mut sim_context::get_sim_context();

    let mut key = ctx.arg_as_utf8_str("key");
    if key.len() == 0 {
        key = "counter1".to_string();
    }

    let count = ctx.get_state("count", &key).unwrap();
    let count: i32 = i32_from(count.as_slice());

    let log_str = format!("query count {}", count);
    let r = count.to_string();

    ctx.log(&log_str);
    ctx.ok(r.as_bytes());
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

// upgrade 更新后的方法
#[no_mangle]
pub extern "C" fn sum() {
    // 获取上下文
    let ctx = &mut sim_context::get_sim_context();

    // 获取传入参数
    let arg1 = ctx.arg_as_utf8_str("arg1");
    let arg2 = ctx.arg_as_utf8_str("arg2");

    let num1 = arg1.parse::<i32>().unwrap();
    let num2 = arg2.parse::<i32>().unwrap();
    let sum = num1 + num2;
    ctx.ok(sum.to_string().as_bytes());
}

#[no_mangle]
pub extern "C" fn functional_verify() {
    sim_context::log("===================================functional_verify start===================================");
    // get
    let ctx = &mut sim_context::get_sim_context();

    let r = ctx.get_state("fact", "fileHash");
    if r.is_err() {
        ctx.log("【error】get_state");
    } else {
        ctx.log("【success】get_state");
    }

    // put
    let code = ctx.put_state("fact", "fileHash", "put str".as_bytes());
    if code != SUCCESS_CODE {
        ctx.log("【error】put_state");
    } else {
        ctx.log("【success】put_state");
    }
    // get
    let r = ctx.get_state("fact", "fileHash");
    if r.is_err() {
        ctx.log("【error】get_state");
    } else {
        ctx.log("【success】get_state");
    }
    let v = r.unwrap();
    let fact_str = std::str::from_utf8(v.as_slice()).unwrap();
    ctx.log(&format!("get state result={:?}", fact_str));
    if fact_str != "put str" {
        ctx.log("【error】get_state");
    } else {
        ctx.log("【success】get_state value verify");
    }

    // del
    let code = ctx.delete_state("fact", "fileHash");
    if code != SUCCESS_CODE {
        ctx.log("delete_state");
    } else {
        ctx.log("【success】 delete_state");
    }

    // get
    let r = ctx.get_state("fact", "fileHash");
    if r.is_err() {
        ctx.log("【error】get_state");
    } else {
        ctx.log("【success】get_state");
    }
    if r.unwrap().len() != 0 {
        ctx.log("【error】get_state");
    } else {
        ctx.log("【success】 after del state get_state value verify");
    }

    // call contract
    let mut ec = EasyCodec::new();
    ec.add_string("file_hash", "LKDFKJW4J2KL3JL34H634");
    ec.add_string("file_name", "亚丁稻城call_contract save.png");
    ec.add_string("time", "16727882774892");

    let mut contract_name = ctx.arg_as_utf8_str("contract_name");
    if contract_name.len() == 0 {
        contract_name = "contract01".to_string();
    }
    let r = ctx.call_contract(&contract_name, "save", ec);
    if r.is_err() {
        ctx.log("【error】 save  call contract");
    } else {
        ctx.log("【success】 save  call contract");
    }

    let mut ec = EasyCodec::new();
    ec.add_string("file_hash", "LKDFKJW4J2KL3JL34H634");
    let r = ctx.call_contract(&contract_name, "find_by_file_hash", ec);
    if r.is_err() {
        ctx.log("【error】 find_by_file_hash  call contract");
    } else {
        ctx.log("【success】 find_by_file_hash  call contract");
    }
    let result = r.unwrap();
    let fact_str = std::str::from_utf8(&result).unwrap();
    ctx.log(&format!(
        " call contract find_by_file_hash result={:?}",
        fact_str
    ));

    sim_context::log("===================================functional_verify marshal start===================================");
    let mut ec = EasyCodec::new();
    ec.add_i32("key1", 123);
    ec.add_string("keyStr", "chainmaker长安链");
    ec.add_bytes("bytes", "2021chainmaker长安链".as_bytes().to_vec());
    ctx.log(&ec.to_json());
    let bytes = ec.marshal();

    let mut ec = EasyCodec::unmarshal(&bytes);
    let bytes = ec.get_bytes("bytes").unwrap();
    let key1 = ec.get_i32("key1").unwrap();
    let key_str = ec.get_string("keyStr").unwrap();

    let msg = format!("bytes {:?}", bytes);
    let msg2 = format!("key1 {:?}", key1);
    let msg3 = format!("key_str {:?}", key_str);
    let msg4 = format!("before 【remove】 len {:?}", ec.len());
    ctx.log(&msg);
    ctx.log(&msg2);
    ctx.log(&msg3);
    ctx.log(&msg4);

    ctx.log("remove key keyStr");
    ec.remove("keyStr");
    let key_str = ec.get_string("keyStr");
    let msg = format!("key_str {:?}", key_str);
    let msg2 = format!("after  【remove】 len {:?}", ec.len());
    ctx.log(&msg);
    ctx.log(&msg2);
    ctx.ok("ok".as_bytes());
    sim_context::log("===================================functional_verify marshal  done===================================");

    sim_context::log("===================================functional_verify done===================================");
}
