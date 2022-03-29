<?php

$verbose = getenv("VERBOSE") == "1";
$v1 = getenv("V1") == "1";
$times = intval(getenv("TIMES"));
$expected = [
    "foo" => "bar",
    "section" => [
        "one" => 2,
        "boolean" => true,
        "subsection" => [
            "an-array" => [
                "of",
                "values"
            ]
        ]
    ]
];

$file = $v1 ? __DIR__ . "/../test.v1.toml" : __DIR__ . "/../test.toml";
$toml = [];
$startTime = 0;
$endTime = 0;
$took = 0;
$totalTime = 0;

for ($i = 0; $i < $times; $i++) {
    $startTime = microtime(true);
    $toml = parse_toml($file);
    $endTime = microtime(true);
    $took = ($endTime - $startTime)*1000;
    $totalTime += $took;
}
// $toml === $expected ? exit(0) : exit(1);
$matched = $toml === $expected;

if ($verbose) {
    printf($matched ? "Values matched\n" : "Values did not match\n");
    if (!$matched) {
        printf("Expected: ");
        var_dump($expected);
        printf("Got: ");
        var_dump($toml);
    }

    printf("Times: %s\n", $times);
    printf("Last parsing took %sms\n", $took);
    printf("Total parsing took %sms\n", $totalTime);
    printf("Average parsing took %sms\n", $totalTime/$times);
} else {
    printf($totalTime/$times);
}

