fn convert_to_seconds(time_str: &str) -> u32 {
    let parts: Vec<&str> = time_str.split(':').collect();
    let hours: u32 = parts[0].parse().unwrap();
    let minutes: u32 = parts[1].parse().unwrap();
    let seconds: u32 = parts[2].parse().unwrap();
    (hours * 60 + minutes) * 60 + seconds
}

fn calculate_total_duration(uncovered_time: &Vec<[u32; 2]>) -> u32 {
    uncovered_time.iter().map(|interval| interval[1] - interval[0]).sum()
}

fn convert_to_time_intervals(input: &Vec<[&str; 2]>) -> Vec<[u32; 2]> {
    input
        .iter()
        .map(|interval| {
            let start_seconds = convert_to_seconds(interval[0]);
            let end_seconds = convert_to_seconds(interval[1]);

            [start_seconds, end_seconds]
        })
        .collect()
}

fn find_uncovered_time(first_array: &Vec<[&str; 2]>, second_array: &Vec<[&str; 2]>) -> Vec<[u32; 2]> {
    let mut uncovered_time: Vec<[u32; 2]> = Vec::new();

    for interval in second_array {
        let start2 = convert_to_seconds(interval[0]);
        let end2 = convert_to_seconds(interval[1]);

        let mut overlap_found = false;

        for interval1 in first_array {
            let start1 = convert_to_seconds(interval1[0]);
            let end1 = convert_to_seconds(interval1[1]);

            if start2 < end1 && end2 > start1 {
                overlap_found = true;

                if start2 < start1 {
                    uncovered_time.push([start2, start1]);
                }

                if end2 > end1 {
                    uncovered_time.push([end1, end2]);
                }

                break;
            }
        }

        if !overlap_found {
            uncovered_time.push([start2, end2]);
        }
    }

    uncovered_time
}
fn main() {
    let first_array = vec![["2:00:10", "2:30:00"], ["5:00:00", "6:30:00"], ["7:00:00", "7:15:00"], ["9:00:00", "10:00:00"]];
    let second_array = vec![["00:20:00","00:50:00"], ["1:00:00", "2:15:00"], ["5:05:00", "5:15:00"], ["6:15:00", "6:45:00"], ["8:30:00", "10:30:00"]];

    let first_intervals = convert_to_time_intervals(&first_array);
    for interval in &first_intervals {
        println!("[{}:{:02}:{:02}, {}:{:02}:{:02}]",
            interval[0] / 3600, (interval[0] % 3600) / 60, interval[0] % 60,
            interval[1] / 3600, (interval[1] % 3600) / 60, interval[1] % 60
        );
    }
    println!("Total duration of first intervals: {} seconds.\n", calculate_total_duration(&first_intervals));

    let result = find_uncovered_time(&first_array, &second_array);
    for interval in &result {
        println!("[{}:{:02}:{:02}, {}:{:02}:{:02}]",
            interval[0] / 3600, (interval[0] % 3600) / 60, interval[0] % 60,
            interval[1] / 3600, (interval[1] % 3600) / 60, interval[1] % 60
        );
    }
    println!("Total uncovered duration of second intervals:{} seconds.", calculate_total_duration(&result));

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_uncovered_time() {
        let first_array = vec![["2:00:10", "2:30:00"], ["5:00:00", "6:30:00"], ["7:00:00", "7:15:00"], ["9:00:00", "10:00:00"]];
        let second_array = vec![["00:20:00","00:50:00"], ["1:00:00", "2:15:00"], ["5:05:00", "5:15:00"], ["6:15:00", "6:45:00"], ["8:30:00", "10:30:00"]];

        let result = find_uncovered_time(&first_array, &second_array);
        println!("fda{:?}", &result);
        assert_eq!(result, vec![[20*60, 50*60], [60*60, 120*60+10], [390*60, 405*60], [510*60, 540*60], [600*60, 630*60]]);
    }
}