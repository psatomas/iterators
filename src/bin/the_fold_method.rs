use std::collections::HashMap;

struct SupportStaff {
    day: String,
    employee: String,
}

fn main() {
    let earnings = [4, 7, 9, 13];

    let sum = earnings.into_iter().fold(0, |total, current| {
        println!("Total: {total}, current: {current}");
        total + current
    });
    println!("{sum}");

    let week = [
        SupportStaff {
            day: String::from("Monday"),
            employee: String::from("Brian"),
        },
        SupportStaff {
            day: String::from("Tuesday"),
            employee: String::from("Cam"),
        },
        SupportStaff {
            day: String::from("Wednesday"),
            employee: String::from("Walter"),
        },
    ];

    let map = week.into_iter().fold(HashMap::new(), |mut data, entry| {
        data.insert(entry.day, entry.employee);
        data
    });
    println!("{map:?}");
}