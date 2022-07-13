/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::sim_context;
use sim_context::*;

// 安装合约时会执行此方法，必须
#[no_mangle]
pub extern "C" fn init_contract() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    ctx.log("init_contract [start]");

    // create performance
    let sql_create_performance = "create table performance_rust (
							id varchar(128) primary key,
							number int
						)";
    let r = ctx_sql.execute_ddl(sql_create_performance);
    match r {
        Err(_) => {
            let msg = "initContract error. create table performance_rust error";
            ctx.log(msg);
            ctx.error(msg);
            return;
        }
        _ => {
            ctx.log("create table performance_rust success.");
        }
    }
    ctx.ok("create table performance_rust success".as_bytes());
    ctx.log("create table performance_rust success");

    
    let sql_insert = format!(
        "insert into performance_rust(id, number) VALUES ('id1', 1)",
    );

    let r = ctx_sql.execute_update(&sql_insert);
    if r.is_err() {
        ctx.log("insert error. ");
        ctx.error("insert error. ");
        return;
    }

    ctx.log("init_contract [end]");
}

// 升级合约时会执行此方法，必须
#[no_mangle]
pub extern "C" fn upgrade() {
    let ctx = &mut sim_context::get_sim_context();
    ctx.log("upgrade [start]");
    ctx.log("upgrade [end]");
}

#[no_mangle]
pub extern "C" fn sql_insert() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let id = ctx.arg_as_utf8_str("id");
    let number = ctx.arg_as_utf8_str("number");

    if id.len() == 0 || number.len() == 0 {
        ctx.log("param id/number is required");
        ctx.error("param id/number is required");
        return;
    }

    // insert
    let sql_insert = format!(
        "insert into performance_rust(id, number) VALUES ({:?}, {:?})",
        id, number
    );

    let r = ctx_sql.execute_update(&sql_insert);
    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    }
}

#[no_mangle]
pub extern "C" fn sql_update() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let id = ctx.arg_as_utf8_str("id");
    let number = ctx.arg_as_utf8_str("number");

    if id.len() == 0 || number.len() == 0 {
        ctx.log("param id/number is required");
        ctx.error("param id/number is required");
        return;
    }

    // insert
    let sql_insert = format!(
        "update  performance_rust set number={:?} where id={:?}",
        number, id
    );

    let r = ctx_sql.execute_update(&sql_insert);
    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    }
}

#[no_mangle]
pub extern "C" fn sql_blank() {
    let ctx = &mut sim_context::get_sim_context();
    ctx.ok("sql blank ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_query_number() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let sql = "select max(number) num from performance_rust";

    let r = ctx_sql.execute_query_one(sql);
    if r.is_err() {
        ctx.log("ExecuteUpdateSingle error");
        ctx.error("-1");
        return;
    }
    let ec = r.unwrap();
    let num = ec.get_string("num");
    ctx.ok(num.unwrap().as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_query_count() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let sql = "select count(*) num from performance_rust";

    let r = ctx_sql.execute_query_one(sql);
    if r.is_err() {
        ctx.log("ExecuteUpdateSingle error");
        ctx.error("-1");
        return;
    }
    let ec = r.unwrap();
    let num = ec.get_string("num");
    ctx.ok(num.unwrap().as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_insert_noparam() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let tx_id = ctx.get_tx_id();
    if tx_id.len() == 0  {
        ctx.log("param tx_id is required");
        ctx.error("param tx_id is required");
        return;
    }

    let sql_insert = format!(
        "insert into performance_rust(id, number) VALUES ({:?}, 1)",
        tx_id
    );

    let r = ctx_sql.execute_update(&sql_insert);
    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    }
}

#[no_mangle]
pub extern "C" fn sql_update_noparam() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    
    let mut number = ctx.arg_as_utf8_str("number");
    if number.len() == 0 {
        number = "2".to_string();
    }
    // insert
    let sql_insert = format!(
        "update  performance_rust set number={} where id='id1'",
        number
    );

    let r = ctx_sql.execute_update(&sql_insert);
    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    }
}