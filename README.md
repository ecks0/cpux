# cpux

View and set CPU parameters on Linux systems.

## Examples

```bash
cpux --cpus 0-5 --freq-min 800mhz --freq-max 4.1ghz --freq-gov powersave
# or
cpux -c 0-5 -n 800mhz -x 4.1ghz -g powersave
#
# * target cpus 0, 1, 2, 3, 4, 5
# * minimum frequency = 800 MHz
# * maximum frequency = 4.1 GHz
# * frequency governor = powersave

cpux --cpus 6-11 --cpu-on false
# or
cpux -c 6-11 -o false
#
# * target cpus 6, 7, 8, 9, 10, 11
# * cpu online = false

cpux --cpus 5,6,9-11 --cpu-on true --pstate-epb 3
# or
cpux -c 5,6,9-11 -o true --pstate-epb 3
#
# * target cpus 5, 6, 9, 10, 11
# * cpu online = true
# * energy/performance bias hint = 3

cpux --cpu-on-each 1100--1100
cpux --cpu-on-each '11 00 -- 11 00'
# or
cpux -O 1100--1100
cpux -O '11 00 -- 11 00'
#
# * set cpus 0, 1, 6, 7 online
# * set cpus 2, 3, 8, 9 offline
# * do not modify cpus 4, 5
```

## Help

```
$ cpux --help
cpux 0.1.2
View and set CPU parameters

USAGE:
    cpux [FLAGS] [OPTIONS] [WAIT]

FLAGS:
    -q, --quiet      Do not print the default summary
        --cpu        Prints CPU online and frequency summary, default
        --freq       Prints CPU frequency governor summary, default if detected
        --pstate     Prints Intel pstate driver summary, default if detected
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cpus <ID>             Target CPUs, default all, e.g. 0,1,2-5,9,12-15
    -o, --cpu-on <BOOL>         CPU online status, true or false (per --cpus)
    -O, --cpu-on-each <BITS>    CPU online status, e.g. 10-1 ⇒ 0=on 1=off 2=skip 3=on
    -g, --freq-gov <GOV>        Frequency governor (per --cpus)
    -x, --freq-max <KHZ>        Max frequency, e.g. 4100000, 4100mhz, 4.1ghz (per --cpus)
    -n, --freq-min <KHZ>        Min frequency, e.g. 4100000, 4100mhz, 4.1ghz (per --cpus)
        --log-level <LEVEL>     Log level, default warn
        --pstate-epb <0-15>     Intel pstate energy/performance bias hint (per --cpus)
        --pstate-epp <PREF>     Intel pstate energy/performance preference (per --cpus)

ARGS:
    <WAIT>    Refresh summary every WAIT seconds
```

## Output

```
$ cpux

  intel_pstate: active
  
  CPU      EPB  EP Pref             EP Prefs
  -------- ---- ----------------    ----------------
  cpu0     3    balance_performance default,performance,balance_performance,balance_power,power
  cpu1     3    balance_performance default,performance,balance_performance,balance_power,power
  cpu2     n/a  n/a                 default,performance,balance_performance,balance_power,power
  cpu3     n/a  n/a                 default,performance,balance_performance,balance_power,power
  cpu4     3    balance_performance default,performance,balance_performance,balance_power,power
  cpu5     3    balance_performance default,performance,balance_performance,balance_power,power
  cpu6     n/a  n/a                 default,performance,balance_performance,balance_power,power
  cpu7     n/a  n/a                 default,performance,balance_performance,balance_power,power
  cpu8     3    balance_performance default,performance,balance_performance,balance_power,power
  cpu9     3    balance_performance default,performance,balance_performance,balance_power,power
  cpu10    n/a  n/a                 default,performance,balance_performance,balance_power,power
  cpu11    n/a  n/a                 default,performance,balance_performance,balance_power,power
  
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
  cpu0    true    900.1 mhz   900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu1    true    2.2 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu2    false   4.0 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu3    false   4.0 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu4    true    1.9 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu5    true    1.1 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu6    false   4.0 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu7    false   4.0 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu8    true    900.0 mhz   900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu9    true    900.4 mhz   900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu10   false   4.0 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  cpu11   false   3.9 ghz     900.0 mhz   4.0 ghz     800.0 mhz   4.1 ghz
  
```
