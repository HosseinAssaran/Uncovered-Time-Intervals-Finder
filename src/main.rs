fn convert_to_minutes(time_str: &str) -> u32 {
    let parts: Vec<&str> = time_str.split(':').collect();
    let hours: u32 = parts[0].parse().unwrap();
    let minutes: u32 = parts[1].parse().unwrap();
    hours * 60 + minutes
}

fn find_uncovered_time(first_array: &Vec<[&str; 2]>, second_array: &Vec<[&str; 2]>) -> (Vec<[u32; 2]>, u32) {
    let mut uncovered_time: Vec<[u32; 2]> = Vec::new();
    let mut total_duration: u32 = 0;

    for interval in second_array {
        let start2 = convert_to_minutes(interval[0]);
        let end2 = convert_to_minutes(interval[1]);

        let mut overlap_found = false;

        for interval1 in first_array {
            let start1 = convert_to_minutes(interval1[0]);
            let end1 = convert_to_minutes(interval1[1]);

            if start2 < end1 && end2 > start1 {
                overlap_found = true;

                if start2 < start1 {
                    uncovered_time.push([start2, start1]);
                    total_duration += start1 - start2;
                }

                if end2 > end1 {
                    uncovered_time.push([end1, end2]);
                    total_duration += end2 - end1;
                }

                break;
            }
        }

        if !overlap_found {
            uncovered_time.push([start2, end2]);
            total_duration += end2 - start2;
        }
    }

    (uncovered_time, total_duration)
}
fn main() {
    let first_array = vec![["2:00", "2:30"], ["5:00", "6:30"], ["7:00", "7:15"], ["9:00", "10:00"],];
    let second_array = vec![["00:20","00:50"], ["1:00", "2:15"], ["5:05", "5:15"], ["6:15", "6:45"], ["8:30", "10:30"]];

    let (result, total_duration) = find_uncovered_time(&first_array, &second_array);

    println!("Total uncovered duration of second array: {}", total_duration); 
    for interval in result {
        println!("[{}:{:02}, {}:{:02}]", interval[0] / 60, interval[0] % 60, interval[1] / 60, interval[1] % 60);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_uncovered_time() {
        let first_array = vec![["2:00", "2:30"], ["5:00", "6:30"], ["7:00", "7:15"], ["9:00", "10:00"]];
        let second_array = vec![["00:20","00:50"], ["1:00", "2:15"], ["5:05", "5:15"], ["6:15", "6:45"], ["8:30", "10:30"]];

        let (result, total_duration) = find_uncovered_time(&first_array, &second_array);

        assert_eq!(result, vec![[20, 50], [60, 120], [390, 405], [510, 540], [600, 630]]);
        assert_eq!(total_duration, 165);  // Total duration of uncovered time
    }
}