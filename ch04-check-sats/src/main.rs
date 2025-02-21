#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = 0;
    let sat_b = 1;
    let sat_c = 2;

    let sat_a_status = check_status(sat_a);
    let sat_b_status = check_status(sat_b);
    let sat_c_status = check_status(sat_c);
    println!(
        "a: {:?}, b:{:?}, c:{:?}",
        sat_a_status, sat_b_status, sat_c_status
    );

    let sat_a_status = check_status(sat_a);
    let sat_b_status = check_status(sat_b);
    let sat_c_status = check_status(sat_c);
    println!(
        "a: {:?}, b:{:?}, c:{:?}",
        sat_a_status, sat_b_status, sat_c_status
    );
}
