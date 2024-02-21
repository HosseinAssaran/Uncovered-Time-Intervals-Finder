<?php

function convertToSeconds($timeStr) {
    $parts = explode(':', $timeStr);
    $hours = (int)$parts[0];
    $minutes = (int)$parts[1];
    $seconds = (int)$parts[2];
    return ($hours * 60 + $minutes) * 60 + $seconds;
}

function calculateTotalDuration($uncoveredTime) {
    return array_reduce($uncoveredTime, function ($carry, $interval) {
        return $carry + ($interval[1] - $interval[0]);
    }, 0);
}

function convertToTimeIntervals($input) {
    return array_map(function ($interval) {
        $startSeconds = convertToSeconds($interval[0]);
        $endSeconds = convertToSeconds($interval[1]);
        return [$startSeconds, $endSeconds];
    }, $input);
}

function findUncoveredTime($firstArray, $secondArray) {
    $uncoveredTime = [];

    foreach ($secondArray as $interval) {
        $start2 = convertToSeconds($interval[0]);
        $end2 = convertToSeconds($interval[1]);

        $overlapFound = false;

        foreach ($firstArray as $interval1) {
            $start1 = convertToSeconds($interval1[0]);
            $end1 = convertToSeconds($interval1[1]);

            if ($start2 < $end1 && $end2 > $start1) {
                $overlapFound = true;

                if ($start2 < $start1) {
                    $uncoveredTime[] = [$start2, $start1];
                }

                if ($end2 > $end1) {
                    $uncoveredTime[] = [$end1, $end2];
                }

                break;
            }
        }

        if (!$overlapFound) {
            $uncoveredTime[] = [$start2, $end2];
        }
    }

    return $uncoveredTime;
}

$firstArray = [
    ["2:00:10", "2:30:00"],
    ["5:00:00", "6:30:00"],
    ["7:00:00", "7:15:00"],
    ["9:00:00", "10:00:00"]
];

$secondArray = [
    ["00:20:00", "00:50:00"],
    ["1:00:00", "2:15:00"],
    ["5:05:00", "5:15:00"],
    ["6:15:00", "6:45:00"],
    ["8:30:00", "10:30:00"]
];
echo "<pre>";
$firstIntervals = convertToTimeIntervals($firstArray);
echo "First Intervals Array:\n";
foreach ($firstIntervals as $interval) {
    printf("[%02d:%02d:%02d, %02d:%02d:%02d]\n",
        $interval[0] / 3600, ($interval[0] % 3600) / 60, $interval[0] % 60,
        $interval[1] / 3600, ($interval[1] % 3600) / 60, $interval[1] % 60
    );
}
echo "Total duration of first intervals: " . calculateTotalDuration($firstIntervals) . " seconds.\n";

$secondIntervals = convertToTimeIntervals($secondArray);
echo "Second Intervals Array:\n";
foreach ($secondIntervals as $interval) {
    printf("[%02d:%02d:%02d, %02d:%02d:%02d]\n",
        $interval[0] / 3600, ($interval[0] % 3600) / 60, $interval[0] % 60,
        $interval[1] / 3600, ($interval[1] % 3600) / 60, $interval[1] % 60
    );
}
echo "Total duration of second intervals: " . calculateTotalDuration($secondIntervals) . " seconds.\n";

echo "Second Uncovered Intervals Array:\n";
$result = findUncoveredTime($firstArray, $secondArray);
foreach ($result as $interval) {
    printf("[%02d:%02d:%02d, %02d:%02d:%02d]\n",
        $interval[0] / 3600, ($interval[0] % 3600) / 60, $interval[0] % 60,
        $interval[1] / 3600, ($interval[1] % 3600) / 60, $interval[1] % 60
    );
}
echo "Total uncovered duration of second intervals: " . calculateTotalDuration($result) . " seconds.\n";

?>
