/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context;
use sim_context::*;

#[no_mangle]
pub extern "C" fn init_contract() {
    sim_context::log("init_contract");
}

#[no_mangle]
pub extern "C" fn upgrade() {
    sim_context::log("upgrade");
}

#[no_mangle]
pub extern "C" fn invoke() {
    sim_context::log("upgrade");
}

#[no_mangle]
pub extern "C" fn save() {
    sim_context::log("============save start============");
    let ctx = &mut sim_context::get_sim_context();

    let txid = ctx.get_tx_id();
    let time = ctx.arg_as_utf8_str("time"); //ms
    let business_type = ctx.arg_as_utf8_str("business_type"); // 业务类型，信贷申请-0；审批-1；复审-2；通过-3；还款-4；
    let org_id = ctx.arg_as_utf8_str("org_id"); // 机构编号，自定义数字标识
    let amount = ctx.arg_as_utf8_str("amount"); // 金额，分为单位
    let pre_txid = ctx.arg_as_utf8_str("pre_txid"); // 前序交易ID

    let mut ec = EasyCodec::new();
    ec.add_string("time", time.as_str());
    ec.add_string("business_type", business_type.as_str());
    ec.add_string("org_id", org_id.as_str());
    ec.add_string("amount", amount.as_str());
    ec.add_string("pre_txid", pre_txid.as_str());
    ec.add_string("txid", txid.as_str());

    ctx.put_state("fact", &txid, ec.marshal().as_slice());
    sim_context::log("============save done============");
}

// 根据交易ID查询前序交易
#[no_mangle]
pub extern "C" fn find_by_pre_txid() {
    sim_context::log("============find_by_pre_txid start============");
    let ctx = &mut sim_context::get_sim_context();
    let mut txid = ctx.arg_as_utf8_str("txid");

    let mut result: String = "[".to_string();
    for _ in 0..5 {
        let r = ctx.get_state("fact", &txid);
        if r.is_err() {
            break;
        }
        let fact_info = r.unwrap();
        if fact_info.len() == 0 {
            break;
        }
        let data = EasyCodec::unmarshal(&fact_info);
        result += &data.to_json();
        result += ",";
        txid = data.get_string("pre_txid").unwrap();
    }
    if result.len() > 1 {
        result.remove(result.len() - 1);
    }
    result += "]";
    ctx.log(&result);
    ctx.ok(result.as_bytes());
    sim_context::log("============find_by_pre_txid done============");
}
