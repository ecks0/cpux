# cpux

View and set CPU and related parameters on Linux.

## Features

- Control from the command line:

  - CPU:
      - online status
  - CPUFreq:
      - frequency governor
      - min frequency
      - max frquency
  - Intel GPU driver:
      - min frequency
      - max frquency
      - boost frquency
  - Intel pstate driver:
      - energy performance bias hint
      - energy performance preference

- Display current values for many data points.

- Straight-forward Rust library.

## Help

```
$ cpux --help
cpux 0.1.5
View and set CPU and related parameters.

USAGE:
    cpux [FLAGS] [OPTIONS] [REFRESH]

FLAGS:
        --cpu        Prints CPU online and frequency summary, default
        --freq       Prints CPU frequency governor summary, default if detected
    -h, --help       Prints help information
        --i915       Prints Intel GPU driver summary, default if detected
        --pstate     Prints Intel pstate driver summary, default if detected
    -q, --quiet      Do not print the default summaries
    -V, --version    Prints version information

OPTIONS:
    -o, --cpu-on <bool>           CPU online status, true or false (per --cpus)
    -O, --cpu-on-each <list>      CPU online status, e.g. 10-1 ⇒ 0=on 1=off 2=skip 3=on
    -c, --cpus <indices>          Target CPUs, default all, e.g. 0,1,2-5,9,12-15
    -g, --freq-gov <gov>          Frequency governor (per --cpus)
    -x, --freq-max <hz>           Max frequency, e.g. 4100mhz, 4.1ghz (per --cpus)
    -n, --freq-min <hz>           Min frequency, e.g. 800mhz, 0.8ghz (per --cpus)
        --i915-freq-boost <hz>    Intel GPU boost frequency, e.g. 1100mhz, 1.1ghz
        --i915-freq-max <hz>      Intel GPU maximum frequency, e.g. 900mhz, 0.9ghz
        --i915-freq-min <hz>      Intel GPU minimum frequency, e.g. 350mhz, 0.35ghz
        --log-level <level>       Log level, default warn, e.g. error|warn|info|debug|trace
        --pstate-epb <0-15>       Intel pstate energy/performance bias hint (per --cpus)
        --pstate-epp <pref>       Intel pstate energy/performance preference (per --cpus)

ARGS:
    <REFRESH>    Refresh summaries every REFRESH seconds
```

## Examples

```bash
cpux --cpus 0-5 --freq-min 800mhz --freq-max 4.1ghz --freq-gov powersave
# or
cpux -c 0-5 -n 800mhz -x 4.1ghz -g powersave
#
# - target cpus 0, 1, 2, 3, 4, 5
# - minimum frequency = 800 MHz
# - maximum frequency = 4.1 GHz
# - frequency governor = powersave

cpux --cpus 6-11 --cpu-on false
# or
cpux -c 6-11 -o false
#
# - target cpus 6, 7, 8, 9, 10, 11
# - cpu online = false

cpux --cpu-on-each 1100--1100
cpux --cpu-on-each '11 00 -- 11 00'
# or
cpux -O 1100--1100
cpux -O '11 00 -- 11 00'
#
# - set cpus 0, 1, 6, 7 online
# - set cpus 2, 3, 8, 9 offline
# - do not modify cpus 4, 5

cpux --cpus 5,6,9-11 --cpu-on true --pstate-epb 3 --i915-freq-min 300mhz --i915-freq-max 800mhz
# or
cpux -c 5,6,9-11 -o true --pstate-epb 3 --i915-freq-min 300mhz --i915-freq-max 0.8ghz
#
# - target cpus 5, 6, 9, 10, 11
# - cpu online = true
# - intel energy/performance bias hint = 3
# - intel gpu min frequency = 300 MHz
# - intel gpu max frequency = 800 MHz
```

## Output

```
$ cpux
  
  intel_pstate: active
  
  CPU      EPB  EP Pref              EP Prefs
  -------- ---- -------------------- --------------------
  cpu0     3    balance_performance  default,performance,balance_performance,balance_power,power
  cpu1     3    balance_performance  default,performance,balance_performance,balance_power,power
  cpu2     n/a  n/a                  default,performance,balance_performance,balance_power,power
  cpu3     n/a  n/a                  default,performance,balance_performance,balance_power,power
  cpu4     3    balance_performance  default,performance,balance_performance,balance_power,power
  cpu5     3    balance_performance  default,performance,balance_performance,balance_power,power
  cpu6     n/a  n/a                  default,performance,balance_performance,balance_power,power
  cpu7     n/a  n/a                  default,performance,balance_performance,balance_power,power
  cpu8     3    balance_performance  default,performance,balance_performance,balance_power,power
  cpu9     3    balance_performance  default,performance,balance_performance,balance_power,power
  cpu10    n/a  n/a                  default,performance,balance_performance,balance_power,power
  cpu11    n/a  n/a                  default,performance,balance_performance,balance_power,power
  
  CPU     Governor         Governors
  ------- ---------------- ----------------
  cpu0    powersave        performance,powersave
  cpu1    powersave        performance,powersave
  cpu2    powersave        performance,powersave
  cpu3    powersave        performance,powersave
  cpu4    powersave        performance,powersave
  cpu5    powersave        performance,powersave
  cpu6    powersave        performance,powersave
  cpu7    powersave        performance,powersave
  cpu8    powersave        performance,powersave
  cpu9    powersave        performance,powersave
  cpu10   powersave        performance,powersave
  cpu11   powersave        performance,powersave
  
  CPU     Online  Cur         Min         Max         Min limit   Max limit
  ------- ------- ----------- ----------- ----------- ----------- -----------
  cpu0    true    1.1 GHz     900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu1    true    900.1 MHz   900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu2    false   3.9 GHz     900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu3    false   4.0 GHz     900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu4    true    900.1 MHz   900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu5    true    900.2 MHz   900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu6    false   3.9 GHz     900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu7    false   4.0 GHz     900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu8    true    900.1 MHz   900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu9    true    900.0 MHz   900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu10   false   3.9 GHz     900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  cpu11   false   4.0 GHz     900.0 MHz   4.0 GHz     800.0 MHz   4.1 GHz
  
  Card   Driver  Actual    Req'd     Min       Max       Boost    Min limit Max limit
  ------ ------- --------  --------  --------  --------  -------- --------- ---------
  card0  i915    350.0 KHz 600.0 KHz 350.0 KHz 900.0 KHz 1.1 MHz  350.0 KHz 1.1 MHz

```

## Wishlist

- hwmon support
  - cpu, etc. temperatures
  - nouveau fan control
  - etc.
- Intel RAPL power control suppot
- Nvidia GPU driver support via `nvml_wrapper`
- AMD CPU support
- AMD GPU support
