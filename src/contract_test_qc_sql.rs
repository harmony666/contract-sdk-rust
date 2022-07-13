/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::sim_context;
use sim_context::*;

#[no_mangle]
pub extern "C" fn init_contract() {
    sim_context::log("[zitao] ========================================start");
    sim_context::log("[zitao] input func: initContract");

    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    let status = ctx.arg_as_utf8_str("status");
    let tblinfo = ctx.arg_as_utf8_str("tblinfo");
    ctx.log(&tblinfo);

    sim_context::log(&format!("[zitao] change initContract[status]: {}", status));
    sim_context::log(&format!(
        "[zitao] change initContract[tblinfo]: {}",
        tblinfo
    ));
    let tblinfo_list = tblinfo.split("##");

    for tblinfo_once in tblinfo_list {
        let r = ctx_sql.execute_ddl(tblinfo_once);
        if r.is_err() {
            let msg = format!(
                "initContract error. resultCode={} tblinfo_once={}",
                r.err().unwrap(),
                tblinfo_once
            );
            ctx.log(&msg);
            ctx.error(&msg);
            return;
        } else {
            ctx.log("create table teacher_gasm success.");
        }
    }

    if status == "success" {
        ctx.ok(format!("create table success:: {}", tblinfo).as_bytes());
        ctx.log("initContract success.");
    } else {
        ctx.error(&format!("create table failure:: {}", tblinfo));
        ctx.log("initContract [end]");
    }
}

#[no_mangle]
pub extern "C" fn upgrade() {
    sim_context::log("[zitao] ========================================start");
    sim_context::log("[zitao] input func: upgrade");

    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    let status = ctx.arg_as_utf8_str("status");
    let tblinfo = ctx.arg_as_utf8_str("tblinfo");
    ctx.log(&tblinfo);

    sim_context::log(&format!("[zitao] change upgrade[status]: {}", status));
    sim_context::log(&format!("[zitao] change upgrade[tblinfo]: {}", tblinfo));
    let tblinfo_list = tblinfo.split("##");

    for tblinfo_once in tblinfo_list {
        let r = ctx_sql.execute_ddl(tblinfo_once);
        if r.is_err() {
            let msg = format!(
                "upgrade error. resultCode={} tblinfo_once={}",
                r.err().unwrap(),
                tblinfo_once
            );
            ctx.log(&msg);
            ctx.error(&msg);
            return;
        } else {
            ctx.log("create table teacher_gasm success.");
        }
    }

    if status == "success" {
        ctx.ok(format!("create table success:: {}", tblinfo).as_bytes());
        ctx.log("upgrade success.");
    } else {
        ctx.error(&format!("create table failure:: {}", tblinfo));
        ctx.log("upgrade [end]");
    }
}

#[no_mangle]
pub extern "C" fn sql_handle_update() {
    sim_context::log("[zitao] ========================================start");
    sim_context::log("[zitao] input func: sql_handle_update");

    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    let status = ctx.arg_as_utf8_str("status");
    let tblinfo = ctx.arg_as_utf8_str("tblinfo");
    ctx.log(&tblinfo);

    sim_context::log(&format!("[zitao] change upgrade[status]: {}", status));
    sim_context::log(&format!("[zitao] change upgrade[tblinfo]: {}", tblinfo));
    let tblinfo_list = tblinfo.split("##");

    for tblinfo_once in tblinfo_list {
        let r = ctx_sql.execute_update(tblinfo_once);
        if r.is_err() {
            let msg = format!(
                "sql_handle_update error. resultCode={} tblinfo_once={}",
                r.err().unwrap(),
                tblinfo_once
            );
            ctx.log(&msg);
            ctx.error(&msg);
            return;
        } else {
            ctx.log(&format!("sql_handle_update update row= {}", r.unwrap()));
        }
    }

    if status == "success" {
        ctx.ok(format!("sql_handle_update table success:: {}", tblinfo).as_bytes());
        ctx.log("sql_handle_update success.");
    } else {
        ctx.error(&format!("sql_handle_update table failure:: {}", tblinfo));
        ctx.log("sql_handle_update [end]");
    }
}

#[no_mangle]
pub extern "C" fn sql_handle_query() {
    sim_context::log("[zitao] ========================================start");
    sim_context::log("[zitao] input func: sql_handle_update");

    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    let status = ctx.arg_as_utf8_str("status");
    let tblinfo = ctx.arg_as_utf8_str("tblinfo");
    ctx.log(&tblinfo);

    sim_context::log(&format!("[zitao] change upgrade[status]: {}", status));
    sim_context::log(&format!("[zitao] change upgrade[tblinfo]: {}", tblinfo));
    let tblinfo_list = tblinfo.split("##");

    let mut result = "".to_string();
    for tblinfo_once in tblinfo_list {
        let r = ctx_sql.execute_query(tblinfo_once);
        if r.is_err() {
            let msg = format!(
                "sql_handle_query error. resultCode={} tblinfo_once={}",
                r.err().unwrap(),
                tblinfo_once
            );
            ctx.log(&msg);
            ctx.error(&msg);
            return;
        } else {
            let rs = r.unwrap();
            while rs.has_next() {
                let r_row = rs.next_row();
                if r_row.is_err() {
                    ctx.log("NextRow error");
                    ctx.error("NextRow error");
                    return;
                }
                let json_str = r_row.unwrap().to_json();
                ctx.log(&format!("NextRow: {}", json_str));
                result.push_str(&json_str);
                result.push_str(",");
            }
            rs.close();
        }
    }
    if result.len() > 0 {
        result.remove(result.len() - 1);
    }

    if status == "success" {
        ctx.ok(format!("[{}]", result).as_bytes());
        ctx.log("sql_handle_query success.");
    } else {
        ctx.error(&format!("sql_handle_query table failure:: {}", tblinfo));
        ctx.log("sql_handle_query [end]");
    }
}

#[no_mangle]
pub extern "C" fn sql_handle_query_one() {
    sim_context::log("[zitao] ========================================start");
    sim_context::log("[zitao] input func: sql_handle_query");

    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    let status = ctx.arg_as_utf8_str("status");
    let tblinfo = ctx.arg_as_utf8_str("tblinfo");
    ctx.log(&tblinfo);

    sim_context::log(&format!("[zitao] change upgrade[status]: {}", status));
    sim_context::log(&format!("[zitao] change upgrade[tblinfo]: {}", tblinfo));
    let tblinfo_list = tblinfo.split("##");

    let mut result = "".to_string();
    for tblinfo_once in tblinfo_list {
        let r = ctx_sql.execute_query_one(tblinfo_once);
        if r.is_err() {
            let msg = format!(
                "sql_handle_query error. resultCode={} tblinfo_once={}",
                r.err().unwrap(),
                tblinfo_once
            );
            ctx.log(&msg);
            ctx.error(&msg);
            return;
        } else {
            let rs = r.unwrap();
            let json_str = rs.to_json();
            ctx.log(&format!("NextRow: {}", json_str));
            result.push_str(&json_str);
            result.push_str(",");
        }
    }
    if result.len() > 0 {
        result.remove(result.len() - 1);
    }

    if status == "success" {
        ctx.ok(format!("[{}]", result).as_bytes());
        ctx.log("sql_handle_query success.");
    } else {
        ctx.error(&format!("sql_handle_query table failure:: {}", tblinfo));
        ctx.log("sql_handle_query [end]");
    }
}
