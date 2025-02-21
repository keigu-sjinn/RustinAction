#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let sat_a_status = check_status(sat_a);
    let sat_b_status = check_status(sat_b);
    let sat_c_status = check_status(sat_c);
    println!(
        "a: {:?}, b:{:?}, c:{:?}",
        sat_a_status, sat_b_status, sat_c_status
    );

    // let sat_a_status = check_status(sat_a);
    // let sat_b_status = check_status(sat_b);
    // let sat_c_status = check_status(sat_c);
    // println!(
    //     "a: {:?}, b:{:?}, c:{:?}",
    //     sat_a_status, sat_b_status, sat_c_status
    // );
}
