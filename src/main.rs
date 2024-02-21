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

fn convert_interval_strings_to_seconds(input: &Vec<[&str; 2]>) -> Vec<[u32; 2]> {
    input
        .iter()
        .map(|interval| {
            let start_seconds = convert_to_seconds(interval[0]);
            let end_seconds = convert_to_seconds(interval[1]);

            [start_seconds, end_seconds]
        })
        .collect()
}

fn find_uncovered_time(first_interval: &Vec<[u32; 2]> , second_interval: &Vec<[u32; 2]> ) -> Vec<[u32; 2]> {
    let mut uncovered_time: Vec<[u32; 2]> = Vec::new();

    for interval in second_interval {
        let start2 = interval[0];
        let end2 = interval[1];

        let mut uncovered = vec![(start2, end2)];

        for interval1 in first_interval {
            let start1 = interval1[0];
            let end1 = interval1[1];

            uncovered = uncovered
                .iter()
                .flat_map(|&(uncovered_start, uncovered_end)| {
                    if uncovered_start >= end1 || uncovered_end <= start1 {
                        // No overlap, keep the uncovered interval
                        vec![(uncovered_start, uncovered_end)]
                    } else {
                        // Overlap, split the uncovered interval into non-overlapping parts
                        let mut parts = Vec::new();
                        if uncovered_start < start1 {
                            parts.push((uncovered_start, start1));
                        }
                        if uncovered_end > end1 {
                            parts.push((end1, uncovered_end));
                        }
                        parts
                    }
                })
                .collect();
        }

        uncovered_time.extend(uncovered.iter().map(|&(start, end)| [start, end]));
    }

    uncovered_time.sort_by_key(|&interval| interval[0]);
    uncovered_time
}

fn main() {
    // Example usage
    let first_array = vec![
        ["2:00:10", "2:30:00"],
        ["5:00:00", "6:30:00"],
        ["7:00:00", "7:15:00"],
        ["9:00:00", "10:00:00"],
        ["11:10:00", "11:30:00"],
        ["11:40:00", "11:50:00"],
        ["12:20:00", "12:30:00"],
    ];
    let second_array = vec![
        ["00:20:00", "00:50:00"],
        ["1:00:00", "2:15:00"],
        ["5:05:00", "5:15:00"],
        ["6:15:00", "6:45:00"],
        ["8:30:00", "10:30:00"],
        ["11:00:00", "13:00:00"],
    ];

    let first_intervals = convert_interval_strings_to_seconds(&first_array);
    println!("First Intervals Array:");
    for interval in &first_intervals {
        println!("[{}:{:02}:{:02}, {}:{:02}:{:02}]",
            interval[0] / 3600, (interval[0] % 3600) / 60, interval[0] % 60,
            interval[1] / 3600, (interval[1] % 3600) / 60, interval[1] % 60
        );
    }
    println!("Total duration of first intervals: {} seconds.\n", calculate_total_duration(&first_intervals));

    let second_intervals = convert_interval_strings_to_seconds(&second_array);
    println!("Second Intervals Array:");
    for interval in &second_intervals {
        println!("[{}:{:02}:{:02}, {}:{:02}:{:02}]",
            interval[0] / 3600, (interval[0] % 3600) / 60, interval[0] % 60,
            interval[1] / 3600, (interval[1] % 3600) / 60, interval[1] % 60
        );
    }
    println!("Total duration of second intervals: {} seconds.\n", calculate_total_duration(&second_intervals));


    println!("Second Uncovered Intervals Array:");
    let result = find_uncovered_time(&first_intervals, &second_intervals);

    for interval in &result {
        println!(
            "[{}:{:02}:{:02}, {}:{:02}:{:02}]",
            interval[0] / 3600,
            (interval[0] % 3600) / 60,
            interval[0] % 60,
            interval[1] / 3600,
            (interval[1] % 3600) / 60,
            interval[1] % 60
        );
    }
println!("Total uncovered duration of second intervals:{} seconds.", calculate_total_duration(&result));

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_uncovered_time() {
        let first_array = vec![
            ["2:00:10", "2:30:00"],
            ["5:00:00", "6:30:00"],
            ["7:00:00", "7:15:00"],
            ["9:00:00", "10:00:00"],
            ["11:10:00", "11:30:00"],
            ["11:40:00", "11:50:00"],
            ["12:20:00", "12:30:00"],
        ];
        let second_array = vec![
            ["00:20:00", "00:50:00"],
            ["1:00:00", "2:15:00"],
            ["5:05:00", "5:15:00"],
            ["6:15:00", "6:45:00"],
            ["8:30:00", "10:30:00"],
            ["11:00:00", "13:00:00"],
        ];
        let first_intervals = convert_interval_strings_to_seconds(&first_array);
        let second_intervals = convert_interval_strings_to_seconds(&second_array);
        let result = find_uncovered_time(&first_intervals, &second_intervals);
        assert_eq!(result, vec![
            [20*60, 50*60], 
            [60*60, 2*60*60+10], 
            [(6*60+30)*60, (6*60+45)*60], 
            [(8*60+30)*60, 9*60*60], 
            [10*60*60, (10*60+30)*60],
            [11*60*60, (11*60+10)*60],
            [(11*60+30)*60, (11*60+40)*60],
            [(11*60+50)*60, (12*60+20)*60],
            [(12*60+30)*60, 13*60*60],
            ]);
    }
}