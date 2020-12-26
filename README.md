# Advent of Code

My playground for messy advent of code solutions

## Usage

``` shell
# run default year
$ cargo run --release

# run specific year
$ cargo run --release -- 2019

# run all years
$ cargo run --release -- all

Advent of Code - 2019

 1-1:                  3295206         0ms         1 allocations        4b  peak memory
 1-2:                  4939939         0ms         1 allocations        4b  peak memory
 2-1:                  8017076         0ms        10 allocations      2.1kb peak memory
 2-2:                     3146         2ms        10 allocations      2.1kb peak memory
 3-1:                      258         0ms        10 allocations     14.0kb peak memory
 3-2:                    12304         0ms        10 allocations     16.0kb peak memory
 4-1:                      979         0ms        10 allocations      8.0kb peak memory
 4-2:                      635         1ms        10 allocations      8.0kb peak memory
 5-1:                  6761139         0ms        12 allocations      9.3kb peak memory
 5-2:                  9217546         0ms        12 allocations      9.3kb peak memory
 6-1:                   300598         0ms        21 allocations     67.0kb peak memory
 6-2:                      520         0ms        37 allocations     74.0kb peak memory
 7-1:                    70597         0ms       134 allocations     11.6kb peak memory
 7-2:                 30872528         0ms       175 allocations     36.0kb peak memory
 8-1:                     2193         0ms         1 allocations        8b  peak memory
 8-2:                                  0ms        10 allocations      0.6kb peak memory

##      ##########  ##    ##  ########  ########
##      ####        ##    ##  ##        ##
  ##  ##  ######    ########  ######    ######
    ##    ##        ##    ##  ##        ##
    ##    ##        ##    ##  ##        ##
    ##    ########  ##    ##  ########  ##

 9-1:               4234906522         0ms        13 allocations     38.3kb peak memory
 9-2:                    60962         6ms        13 allocations     38.3kb peak memory
10-1:                      309        14ms        22 allocations    137.9kb peak memory
10-2:                      416        15ms        43 allocations    137.9kb peak memory
11-1:                     2319         2ms        24 allocations    129.2kb peak memory
11-2:                                  0ms        29 allocations     39.9kb peak memory

  ##    ##  ########  ######    ######    ######    ########    ####        ####
  ##    ##  ##        ##    ##  ##    ##  ##    ##  ##        ##    ##        ##
  ##    ##  ######    ##    ##  ##    ##  ##    ##  ######    ##              ##
  ##    ##  ##        ######    ######    ######    ##        ##  ####        ##
  ##    ##  ##        ##  ##    ##        ##  ##    ##        ##    ##  ##    ##
    ####    ########  ##    ##  ##        ##    ##  ##          ######    ####

12-1:                     7928         0ms        12 allocations      192b  peak memory
12-2:          518311327635164         5ms        32 allocations      0.8kb peak memory
13-1:                      273         0ms        15 allocations     57.5kb peak memory
13-2:                    13140        17ms        15 allocations    115.0kb peak memory
14-1:                  1185296         0ms        70 allocations     11.0kb peak memory
14-2:                  1376631         6ms        70 allocations     11.0kb peak memory
15-1:                      254       108ms     28064 allocations     64.5kb peak memory
15-2:                      268       104ms     27381 allocations     64.5kb peak memory
16-1:                 68317988         2ms       663 allocations      0.8mb peak memory
16-2:                 53850800        58ms        21 allocations      1.0mb peak memory
17-1:                     3292         0ms        24 allocations     69.2kb peak memory
17-2:                   651043         1ms       787 allocations     69.2kb peak memory
Total duration                       352ms


Advent of Code - 2020

 1-1:                  1007331         0ms         8 allocations      3.4kb peak memory
 1-2:                 48914340         0ms         8 allocations      3.4kb peak memory
 2-1:                      580         0ms        11 allocations     40.0kb peak memory
 2-2:                      611         0ms        11 allocations     40.0kb peak memory
 3-1:                      294         0ms       979 allocations     23.4kb peak memory
 3-2:               5774564250         0ms       980 allocations     23.4kb peak memory
 4-1:                      254         0ms       733 allocations    149.4kb peak memory
 4-2:                      184         0ms       733 allocations    149.4kb peak memory
 5-1:                      955         0ms         1 allocations        8b  peak memory
 5-2:                      569         0ms        10 allocations     13.5kb peak memory
 6-1:                     6259         0ms      1442 allocations      272b  peak memory
 6-2:                     3178         0ms      1442 allocations      464b  peak memory
 7-1:                      248         0ms      3664 allocations    182.3kb peak memory
 7-2:                    57281         0ms      6990 allocations    153.8kb peak memory
 8-1:                     1487         0ms        18 allocations     19.4kb peak memory
 8-2:                     1607         0ms        24 allocations     20.0kb peak memory
 9-1:                466456641         0ms         6 allocations      256b  peak memory
 9-2:                 55732936         0ms        13 allocations      4.0kb peak memory
10-1:                     3034         0ms         9 allocations      1.9kb peak memory
10-2:          259172170858496         0ms        15 allocations      7.1kb peak memory
11-1:                     2093        12ms         3 allocations     16.1kb peak memory
11-2:                     1862        30ms         3 allocations     16.1kb peak memory
12-1:                     2057         0ms        11 allocations      8.0kb peak memory
12-2:                    71504         0ms        11 allocations      8.0kb peak memory
13-1:                     2045         0ms         1 allocations        4b  peak memory
13-2:          402251700208309         0ms         4 allocations      256b  peak memory
14-1:           12512013221615         0ms         9 allocations     12.8kb peak memory
14-2:            3905642473893         3ms       117 allocations      3.2mb peak memory
15-1:                      206         0ms         2 allocations      7.9kb peak memory
15-2:                      955       400ms         2 allocations    114.4mb peak memory
16-1:                    27870         0ms        55 allocations      1.6kb peak memory
16-2:            3173135507987         0ms      1546 allocations     58.6kb peak memory
17-1:                      213         2ms        46 allocations      8.2kb peak memory
17-2:                     1624       100ms        60 allocations    119.0kb peak memory
18-1:           36382392389406         0ms      1497 allocations      4.1kb peak memory
18-2:          381107029777968         0ms      1497 allocations      4.1kb peak memory
19-1:                      182         0ms      1621 allocations     29.0kb peak memory
19-2:                      334       986ms    209478 allocations    249.7kb peak memory
20-1:          107399567124539         0ms       152 allocations     30.1kb peak memory
20-2:                     1555         2ms       468 allocations     67.5kb peak memory
21-1:                     1815         0ms       322 allocations     15.1kb peak memory
21-2:                                  0ms       321 allocations      7.9kb peak memory
kllgt,jrnqx,ljvx,zxstb,gnbxs,mhtc,hfdxb,hbfnkq
22-1:                    32033         0ms         1 allocations        8b  peak memory
22-2:                    34901        87ms     18159 allocations      0.7mb peak memory
23-1:                 76952348         0ms         2 allocations       72b  peak memory
23-2:              72772522064       197ms         2 allocations      7.6mb peak memory
24-1:                      346         0ms      1163 allocations     36.3kb peak memory
24-2:                     3802       751ms      1165 allocations     23.5mb peak memory
25-1:                  3015200        25ms         1 allocations        8b  peak memory
25-2:             I did it!!!!         0ms         1 allocations       16b  peak memory
Total duration                      2608ms


Overall duration                    2960ms
```
