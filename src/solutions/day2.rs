pub fn get_answer1() -> i64 {
    let input = get_input();
    let ids = input.split(',');
    let mut invalid_count = 0;

    for id in ids {
        let parts: Vec<&str> = id.split('-').collect();
        let start: i64 = parts[0].parse().expect("not a number");
        let end: i64 = parts[1].parse().expect("not a number");

        for n in start..end+1{
            let s = n.to_string();
            invalid_count += has_error1(&s);
        }
    }
    invalid_count
}

pub fn get_answer2() -> i64 {
    let input = get_input();
    let ids = input.split(',');
    let mut invalid_count = 0;

    for id in ids {
        let parts: Vec<&str> = id.split('-').collect();
        let start: i64 = parts[0].parse().expect("not a number");
        let end: i64 = parts[1].parse().expect("not a number");

        for n in start..end+1{
            let s = n.to_string();
            invalid_count += has_error2(&s);
        }
    }
    invalid_count
}

fn has_error1(line: &str) -> i64 {
    let length = line.len();
    let val: i64 = line.parse().expect("failed to convert in helper");

    for n in 0..length {
        let data = line.split_at(n);
        if data.0 == data.1 {
            return val;
        }
    }

    0
}

fn has_error2(line: &str) -> i64 {
    let length = line.len();
    let val: i64 = line.parse().expect("failed to convert in helper");

    for n in 1..length {
        let data = line.split_at(n);
        if data.1.len() % data.0.len() != 0 {
            continue;
        }

        let req_matches = data.1.len() / data.0.len();

        if data.1.matches(data.0).count() == req_matches {
            return val;
        }
    }

    0
}


fn get_input() -> String {
    "3737332285-3737422568,5858547751-5858626020,166911-236630,15329757-15423690,753995-801224,1-20,2180484-2259220,24-47,73630108-73867501,4052222-4199117,9226851880-9226945212,7337-24735,555454-591466,7777695646-7777817695,1070-2489,81504542-81618752,2584-6199,8857860-8922218,979959461-980003045,49-128,109907-161935,53514821-53703445,362278-509285,151-286,625491-681593,7715704912-7715863357,29210-60779,3287787-3395869,501-921,979760-1021259".to_string()
}

// fn get_example_input() -> String{
//     "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
// }

