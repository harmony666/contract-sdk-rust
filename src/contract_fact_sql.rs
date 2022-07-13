/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

use crate::easycodec::*;
use crate::sim_context;
use sim_context::*;

// 安装合约时会执行此方法，必须
#[no_mangle]
pub extern "C" fn init_contract() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    // create teacher
    let sql_create_teacher = "create table teacher (
							id varchar(128) primary key,
							name varchar(64) DEFAULT ''
						)";
    let r = ctx_sql.execute_ddl(sql_create_teacher);
    match r {
        Err(_) => {
            let msg = "initContract error. create table teacher error";
            ctx.log(msg);
            ctx.error(msg);
            return;
        }
        _ => {
            ctx.log("create table teacher success.");
        }
    }

    // create student
    let sql_create_student = "create table student (
					id varchar(128) primary key,
					teacher_id varchar(128),
					name varchar(64) DEFAULT '',
					age int DEFAULT 0,
					score int DEFAULT 0,
					id_card_no varchar(19) DEFAULT '',
					attend_school_time date
				)";
    let r = ctx_sql.execute_ddl(sql_create_student);
    match r {
        Err(_) => {
            let msg = "initContract error. create table student error";
            ctx.log(msg);
            ctx.error(msg);
            return;
        }
        _ => {
            ctx.log("create table student success.");
        }
    }

    ctx.ok("create table student、teacher success".as_bytes());
    ctx.log("initContract success.");
}

// 升级合约时会执行此方法，必须
#[no_mangle]
pub extern "C" fn upgrade() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let sql_add_column = "ALTER TABLE student ADD address varchar(255)";

    let r = ctx_sql.execute_ddl(sql_add_column);
    if r.is_err() {
        ctx.log("upgrade error.");
        ctx.error("upgrade error.");
    } else {
        ctx.log("upgrade success.");
        ctx.ok("upgrade success.".as_bytes());
    }
}

#[no_mangle]
pub extern "C" fn sql_insert() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let id = ctx.arg_as_utf8_str("id");
    let age = ctx.arg_as_utf8_str("age");
    let name = ctx.arg_as_utf8_str("name");
    let id_card_no = ctx.arg_as_utf8_str("id_card_no");

    if id.len() == 0 || age.len() == 0 {
        ctx.log("param id/age is required");
        ctx.error("param id/age is required");
        return;
    }

    // insert
    let sql_insert = format!(
        "insert into student(id, name, age, id_card_no) VALUES ('{}', '{}', {}, '{}')",
        id, name, age, id_card_no
    );

    ctx.log("sql_insert");
    ctx.log(&sql_insert);
    let r = ctx_sql.execute_update(&sql_insert);
    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    }

    ctx.log("execute_update_single ok");
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_query_by_id() {
    sim_context::log("sql_query_by_id");
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let id = ctx.arg_as_utf8_str("id");
    if id.len() == 0 {
        ctx.log("param id is required");
        ctx.error("param id is required");
        return;
    }

    let mut msg = "sql_query_by_id id:".to_string();
    msg.push_str(&id);
    ctx.log(&msg);

    let sql_query = format!(
        "select id, name, age, id_card_no from student where id='{}'",
        id
    );

    let r = ctx_sql.execute_query_one(&sql_query);
    if r.is_err() {
        ctx.log("execute_query error. ");
        ctx.error("execute_query error. ");
        return;
    }
    let ec = r.unwrap();
    let mut msg = "sql_query_by_id ok result:".to_string();
    msg.push_str(&ec.to_json());
    ctx.log(&msg);
    ctx.ok(ec.to_json().as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_query_range_of_age() {
    sim_context::log("sql_query_range_of_age");
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let max_age = ctx.arg_as_utf8_str("max_age");
    let min_age = ctx.arg_as_utf8_str("min_age");
    if max_age.len() == 0 || min_age.len() == 0 {
        ctx.log("param max_age/min_age is required");
        ctx.error("param max_age/min_age is required");
        return;
    }

    let sql_query = format!(
        "select id, name, age, id_card_no from student where age>{} and age<{}",
        min_age, max_age
    );

    let r = ctx_sql.execute_query(&sql_query);
    if r.is_err() {
        ctx.log("execute_query error. ");
        ctx.error("execute_query error. ");
        return;
    }
    let result_set = r.unwrap();
    let mut result = "".to_string();
    while result_set.has_next() {
        let ec = result_set.next_row().unwrap();
        ctx.log(&ec.to_json());
        result.push_str(ec.to_json().as_str());
    }
    result_set.close();

    ctx.log("sql_query_range_of_age ok ");
    ctx.ok(result.as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_update() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let name = ctx.arg_as_utf8_str("name");

    // insert
    let sql_insert = format!("update student set name='{}' ", name);

    let r = ctx_sql.execute_update(&sql_insert);

    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    } else {
        ctx.log("execute_update_single success. ");
    }
    ctx.log("sql_update ok");
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_update_rollback_save_point() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let name = ctx.arg_as_utf8_str("name");
    let tx_id = ctx.get_tx_id();

    // insert
    let sql_update = format!("update student set name='{}' ", name);
    ctx.log(&sql_update);

    let r = ctx_sql.execute_update(&sql_update);
    if r.is_err() {
        ctx.log("execute_update_single sql_update error. ");
        ctx.error("execute_update_single sql_update error. ");
        return;
    } else {
        ctx.log("execute_update_single sql_update success. ");
    }

    let sql_insert = format!("insert into student(id, name, age, id_card_no) VALUES ('{}', 'Tom', '18', '409787417841395')", tx_id);

    ctx.log(&sql_insert);
    let r = ctx_sql.execute_update(&sql_insert);

    if r.is_err() {
        ctx.log("execute_update_single sql_insert error. ");
        ctx.error("execute_update_single sql_insert error. ");
        return;
    } else {
        ctx.log("execute_update_single sql_insert success. ");
    }
    ctx.log("sql_update ok");
    ctx.error("save point test");
}

#[no_mangle]
pub extern "C" fn sql_delete() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    let id = ctx.arg_as_utf8_str("id");

    // insert
    let sql_delete = format!("delete from student where id='{}'", id);

    let r = ctx_sql.execute_update(&sql_delete);

    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    } else {
        ctx.log("execute_update_single success. ");
    }
    ctx.log("sql_delete ok");
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_cross_call() {
    let ctx = &mut sim_context::get_sim_context();

    let mut contract_name = ctx.arg_as_utf8_str("contract_name");
    let mut min_age = ctx.arg_as_utf8_str("min_age");
    let mut max_age = ctx.arg_as_utf8_str("max_age");

    if contract_name.len() == 0 {
        contract_name = "contract01".to_string();
    }
    if min_age.len() == 0 {
        min_age = "0".to_string();
    }
    if max_age.len() == 0 {
        max_age = "20".to_string();
    }

    let mut ec = EasyCodec::new();
    ec.add_string("min_age", &min_age);
    ec.add_string("max_age", &max_age);

    let r = ctx.call_contract(&contract_name, "sql_query_range_of_age", ec);
    if r.is_err() {
        ctx.log("【error】 sql_cross_call sql_query_range_of_age contract");
        ctx.error("!!!!!!!!!!!!!!!!!!!test fail.!!!!!!!!!!!!!!!!!!!");
        return;
    } else {
        ctx.log("【success】 sql_cross_call sql_query_range_of_age contract");
    }
    let result = r.unwrap();
    let fact_str = std::str::from_utf8(&result).unwrap();
    ctx.log(&format!(
        " call contract find_by_file_hash result={}",
        fact_str
    ));
    ctx.ok(fact_str.as_bytes());
}

// Exception test Error test
// 只能在init_contract  upgrade执行的 create table index view
// 1、建表、索引、视图等DDL语句只能在合约安装init_contract 和合约升级upgrade中使用。
#[no_mangle]
pub extern "C" fn sql_execute_ddl() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();
    // create
    let sql_create_teacher = "create table teacher_normal_ddl (
        id varchar(128) primary key,
        name varchar(64) DEFAULT ''
    )";
    let r = ctx_sql.execute_ddl(sql_create_teacher);
    match r {
        Err(_) => {
            let msg = "sql_execute_ddl execute error. 符合预期In line with expectations";
            ctx.log(msg);
            ctx.error(msg);
            return;
        }
        _ => {
            ctx.log("sql_execute_ddl sql_drop execute success. 不符合预期 Not as expected");
        }
    }

    // create
    let sql_create_teacher = "create table teacher_normal_ddl (
        id varchar(128) primary key,
        name varchar(64) DEFAULT ''
    )";
    let r = ctx_sql.execute_ddl(sql_create_teacher);
    match r {
        Err(_) => {
            let msg = "sql_execute_ddl sql_drop execute error. 符合预期In line with expectations";
            ctx.log(msg);
            ctx.error(msg);
        }
        _ => {
            ctx.log("sql_execute_ddl sql_drop execute success. 不符合预期 Not as expected");
        }
    }

    let sql_drop = "drop table student";
    let r = ctx_sql.execute_ddl(sql_drop);
    match r {
        Err(_) => {
            let msg = "sql_execute_ddl sql_drop execute error. 符合预期In line with expectations";
            ctx.log(msg);
            ctx.error(msg);
        }
        _ => {
            ctx.log("sql_execute_ddl sql_drop execute success. 不符合预期 Not as expected");
        }
    }

    let sql_drop = "drop table teacher";
    let r = ctx_sql.execute_ddl(sql_drop);
    match r {
        Err(_) => {
            let msg = "sql_execute_ddl execute error. 符合预期In line with expectations";
            ctx.log(msg);
            ctx.error(msg);
        }
        _ => {
            ctx.log("sql_execute_ddl execute success. 不符合预期 Not as expected");
        }
    }

    let sql_alter = "ALTER TABLE teacher MODIFY COLUMN name varchar(129) ";
    let r = ctx_sql.execute_ddl(sql_alter);
    match r {
        Err(_) => {
            let msg = "sql_execute_ddl sql_alter execute error. 符合预期In line with expectations";
            ctx.log(msg);
            ctx.error(msg);
        }
        _ => {
            ctx.log("sql_execute_ddl sql_alter execute success. 不符合预期 Not as expected");
        }
    }
    ctx.ok("ok".as_bytes());
}

// 2、SQL中，禁止跨数据库操作，无需指定数据库名。比如select * from db.table 是禁止的； use db;是禁止的。
#[no_mangle]
pub extern "C" fn sql_dbname_table_name() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_delete = format!("delete from mysql.student where id='123'");

    let r = ctx_sql.execute_update(&sql_delete);

    if r.is_err() {
        let msg = "sql_dbname_table_name execute_update execute error. delete from mysql.student where id='123' 符合预期 In line with expectations";
        ctx.log(msg);
        ctx.error(msg);
    } else {
        ctx.log("sql_dbname_table_name execute_update execute success. delete from mysql.student where id='123' 不符合预期 Not as expected");
    }

    // insert
    let sql_delete = format!("use statedb_chain1");

    let r = ctx_sql.execute_update(&sql_delete);

    if r.is_err() {
        let msg = "sql_dbname_table_name execute_update execute error. use statedb_chain1 符合预期 In line with expectations";
        ctx.log(msg);
        ctx.error(msg);
    } else {
        ctx.log("sql_dbname_table_name execute_update execute success. use statedb_chain1 不符合预期 Not as expected");
    }
    ctx.ok("ok".as_bytes());
}

// 3、SQL中，禁止使用事务相关操作的语句，比如commit 、rollback等，事务由ChainMaker框架自动控制。
#[no_mangle]
pub extern "C" fn sql_execute_commit() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_delete = format!("commit;");

    let r = ctx_sql.execute_update(&sql_delete);

    if r.is_err() {
        let msg = "sql_dbname_table_name execute_update execute error. commit 符合预期 In line with expectations";
        ctx.log(msg);
        ctx.error(msg);
    } else {
        ctx.log("sql_dbname_table_name execute_update execute success. commit 不符合预期 Not as expected");
    }

    // insert
    let sql_delete = format!("rollback;");

    let r = ctx_sql.execute_update(&sql_delete);

    if r.is_err() {
        let msg = "sql_dbname_table_name execute_update execute error. rollback 符合预期 In line with expectations";
        ctx.log(msg);
        ctx.error(msg);
    } else {
        ctx.log("sql_dbname_table_name execute_update execute success. rollback 不符合预期 Not as expected");
    }
    ctx.ok("ok".as_bytes());
}

// 4、SQL中，禁止使用随机数、获得系统时间等不确定性函数，这些函数在不同节点产生的结果可能不一样，导致合约执行结果无法达成共识。
#[no_mangle]
pub extern "C" fn sql_random_key() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_insert = format!("update student set name='a' where id=now() ");

    let r = ctx_sql.execute_update(&sql_insert);

    if r.is_err() {
        ctx.log("execute_update_single error. ");
        ctx.error("execute_update_single error. ");
        return;
    } else {
        ctx.log("execute_update_single success. ");
    }
    ctx.log("sql_update ok");
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_random_str() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_insert = format!("update student set name='now()1'");

    let r = ctx_sql.execute_update(&sql_insert);

    if r.is_err() {
        ctx.log("sql_random_str error. ");
        ctx.error("sql_random_str error. ");
        return;
    } else {
        ctx.log("sql_random_str success. ");
    }

    // insert
    let sql_insert = format!("update student set name=\"now()2\"");

    let r = ctx_sql.execute_update(&sql_insert);

    if r.is_err() {
        ctx.log("sql_random_str error. ");
        ctx.error("sql_random_str error. ");
        return;
    } else {
        ctx.log("sql_random_str success. ");
    }
    ctx.log("sql_update ok");
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_random_query_str() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_insert = format!("select * from student where name='now()1'");

    let r = ctx_sql.execute_query(&sql_insert);

    if r.is_err() {
        ctx.log("sql_random_str error. ");
        ctx.error("sql_random_str error. ");
        return;
    } else {
        ctx.log("sql_random_str success. ");
        r.unwrap().close();
    }

    ctx.log("sql_update ok");
    ctx.ok("ok".as_bytes());
}

// 5、SQL中，禁止多条SQL拼接成一个SQL字符串传入。
#[no_mangle]
pub extern "C" fn sql_multi_sql() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_insert = format!("update student set name='111';update student set name='222';");
    let r = ctx_sql.execute_update(&sql_insert);

    if r.is_err() {
        let msg = "sql_dbname_table_name execute_update execute error. commit 符合预期 In line with expectations";
        ctx.log(msg);
        ctx.error(msg);
    } else {
        ctx.log("sql_dbname_table_name execute_update execute success. commit 不符合预期 Not as expected");
    }
    ctx.ok("ok".as_bytes());
}

// 6、禁止建立、修改或删除表名为“state_infos”的表，这是系统自带的提供KV数据存储的表，用于存放PutState函数对应的数据。
#[no_mangle]
pub extern "C" fn sql_update_state_info() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_insert = format!("update STATE_INFOS set block_height='100'");
    let r = ctx_sql.execute_update(&sql_insert);

    if r.is_err() {
        let msg = "sql_dbname_table_name execute_update execute error. commit 符合预期 In line with expectations";
        ctx.log(msg);
        ctx.error(msg);
    } else {
        ctx.log("sql_dbname_table_name execute_update execute success. commit 不符合预期 Not as expected");
    }
    ctx.ok("ok".as_bytes());
}

#[no_mangle]
pub extern "C" fn sql_query_state_info() {
    let ctx = &mut sim_context::get_sim_context();
    let ctx_sql = ctx.get_sql_sim_context();

    // insert
    let sql_insert = format!("select * from STATE_INFOS");
    let r = ctx_sql.execute_query(&sql_insert);

    if r.is_err() {
        let msg = "sql_dbname_table_name execute_update execute error. commit 符合预期 In line with expectations";
        ctx.log(msg);
        ctx.error(msg);
    } else {
        ctx.log("sql_dbname_table_name execute_update execute success. commit 不符合预期 Not as expected");
    }
    ctx.ok("ok".as_bytes());
}

