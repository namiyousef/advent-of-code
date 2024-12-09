use std::fs;

fn main () {

    let test = vec![1,2,3,4];

    let file_contents = fs::read_to_string("assets/input.txt").expect(
        "Was not able to read the file"
    );

    let mut num_safe_reports = 0;
    for report_string in file_contents.lines() {
        let report: Vec<i32> = (
            report_string
            .split_whitespace()
            .map(
                |level_str| level_str.parse().expect("Could not parse {level_str} into an integer")
            )
            .collect()
        );

        if is_report_safe(report) {
            num_safe_reports += 1;
        }


    }


    println!("Num safe reports: {num_safe_reports}")

}

/// Determines if a report is safe
fn is_report_safe (report: Vec<i32>) -> bool {
    let mut increasing = report[1] - report[0] > 0;
    for index in 0..report.len() {
        if index != 0 {
            let diff = report[index] - report[index - 1];
            if (diff > 0) != increasing || diff == 0 {
                return false
            }

            if diff.abs() > 3 {
                return false
            }
        }
    }
    return true
}