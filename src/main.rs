/// 
/// Copyright (C) BABEC. All rights reserved.
/// 
/// SPDX-License-Identifier: Apache-2.0
/// 

/// test command
/// cargo run --package chainmaker-contract --bin chainmaker-contract
fn main() {
    let sql_insert = format!(
        "insert into student(id, name, age, id_card_no) VALUES ({}, {:?}, {:?}, {:?})",
        "id", "name", "age", "id_card_no"
    );
    println!("{}", sql_insert);
} // -------------+-- max, x over
