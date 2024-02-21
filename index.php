<?php

function convert_to_seconds($time_str) {
    $parts = explode(':', $time_str);
    $hours = (int)$parts[0];
    $minutes = (int)$parts[1];
    $seconds = (int)$parts[2];
    return ($hours * 60 + $minutes) * 60 + $seconds;
}

function calculate_total_duration($uncovered_time) {
    return array_sum(array_map(function ($interval) {
        return $interval[1] - $interval[0];
    }, $uncovered_time));
}

function convert_interval_strings_to_seconds($input) {
    return array_map(function ($interval) {
        $start_seconds = convert_to_seconds($interval[0]);
        $end_seconds = convert_to_seconds($interval[1]);
        return [$start_seconds, $end_seconds];
    }, $input);
}

function find_uncovered_time($first_interval, $second_interval) {
    $uncovered_time = [];

    foreach ($second_interval as $interval) {
        $start2 = $interval[0];
        $end2 = $interval[1];

        $uncovered = [[$start2, $end2]];

        foreach ($first_interval as $interval1) {
            $start1 = $interval1[0];
            $end1 = $interval1[1];

            $uncovered = array_reduce($uncovered, function ($acc, $uncovered_interval) use ($start1, $end1) {
                $uncovered_start = $uncovered_interval[0];
                $uncovered_end = $uncovered_interval[1];

                if ($uncovered_start >= $end1 || $uncovered_end <= $start1) {
                    // No overlap, keep the uncovered interval
                    $acc[] = [$uncovered_start, $uncovered_end];
                } else {
                    // Overlap, split the uncovered interval into non-overlapping parts
                    if ($uncovered_start < $start1) {
                        $acc[] = [$uncovered_start, $start1];
                    }
                    if ($uncovered_end > $end1) {
                        $acc[] = [$end1, $uncovered_end];
                    }
                }

                return $acc;
            }, []);
        }

        $uncovered_time = array_merge($uncovered_time, $uncovered);
    }

    usort($uncovered_time, function ($a, $b) {
        return $a[0] - $b[0];
    });

    return $uncovered_time;
}

 echo "<pre>";
 
// Example usage
$first_array = [
    ["2:00:10", "2:30:00"],
    ["5:00:00", "6:30:00"],
    ["7:00:00", "7:15:00"],
    ["9:00:00", "10:00:00"],
    ["11:10:00", "11:30:00"],
    ["11:40:00", "11:50:00"],
    ["12:20:00", "12:30:00"],
];

$second_array = [
    ["00:20:00", "00:50:00"],
    ["1:00:00", "2:15:00"],
    ["5:05:00", "5:15:00"],
    ["6:15:00", "6:45:00"],
    ["8:30:00", "10:30:00"],
    ["11:00:00", "13:00:00"],
];

$first_intervals = convert_interval_strings_to_seconds($first_array);
echo "First Intervals Array:\n";
foreach ($first_intervals as $interval) {
    printf("[%d:%02d:%02d, %d:%02d:%02d]\n",
        $interval[0] / 3600, ($interval[0] % 3600) / 60, $interval[0] % 60,
        $interval[1] / 3600, ($interval[1] % 3600) / 60, $interval[1] % 60
    );
}
echo "Total duration of first intervals: " . calculate_total_duration($first_intervals) . " seconds.\n";

$second_intervals = convert_interval_strings_to_seconds($second_array);
echo "Second Intervals Array:\n";
foreach ($second_intervals as $interval) {
    printf("[%d:%02d:%02d, %d:%02d:%02d]\n",
        $interval[0] / 3600, ($interval[0] % 3600) / 60, $interval[0] % 60,
        $interval[1] / 3600, ($interval[1] % 3600) / 60, $interval[1] % 60
    );
}
echo "Total duration of second intervals: " . calculate_total_duration($second_intervals) . " seconds.\n";

echo "Second Uncovered Intervals Array:\n";
$result = find_uncovered_time($first_intervals, $second_intervals);
foreach ($result as $interval) {
    printf("[%d:%02d:%02d, %d:%02d:%02d]\n",
        $interval[0] / 3600, ($interval[0] % 3600) / 60, $interval[0] % 60,
        $interval[1] / 3600, ($interval[1] % 3600) / 60, $interval[1] % 60
    );
}
echo "Total uncovered duration of second intervals: " . calculate_total_duration($result) . " seconds.\n";
