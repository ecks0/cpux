# cpux

View and set CPU parameters on Linux systems using sysfs.

## Examples

```bash
cpux --cpus 0-5 --freq-min 800mhz --freq-max 4.1ghz --freq-gov powersave

cpux --cpus 6-11 --online false

cpux --cpus 5,6,9-11 --online true

# turn on cpus 0, 1, 4, 5
# turn off cpus 2, 3, 6, 7
cpux --online-each 11001100
cpux --online-each '11 00 11 00'

# turn on cpus 0, 1
# skip cpus 2, 3
# turn off cpus 4, 5
cpux --online-each 11--00
cpux --online-each '11 -- 00'
```

The same as above, with short arguments.

```bash
cpux -c 0-5 -n 800mhz -x 4.1ghz -g powersave

cpux -c 6-11 -o false

cpux -c 5,6,9-11 -o true
```

## Help

```
$ cpux --help
cpux 0.1.1
View and set CPU parameters

USAGE:
    cpux [FLAGS] [OPTIONS] [WAIT]

FLAGS:
    -q, --quiet      Do not print summary
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cpus <ID>              Target CPUs, default all, e.g. 0,1,2-5,9,12-15
        --epb-hint <0-15>        Energy/performance bias hint (per --cpus)
        --freq-ep-pref <PREF>    Energy/performance preference (per --cpus)
    -g, --freq-gov <GOV>         Frequency governor (per --cpus)
    -x, --freq-max <KHZ>         Max frequency, e.g. 4100000, 4100mhz, 4.1ghz (per --cpus)
    -n, --freq-min <KHZ>         Min frequency, e.g. 4100000, 4100mhz, 4.1ghz (per --cpus)
    -l, --log-level <LEVEL>      Log level, default warn
    -o, --online <BOOL>          CPU online status, true or false (per --cpus)
        --online-cpus <BITS>     CPU online status, e.g. 10-1 ⇒ 0=on 1=off 2=skip 3=on

ARGS:
    <WAIT>    Refresh summary every WAIT seconds
```

## Output

```
$ cpux

CPU      EP Pref             EP Prefs
-------- ----------------    ----------------
cpu0     balance_performance default,performance,balance_performance,balance_power,power
cpu1     balance_performance default,performance,balance_performance,balance_power,power
cpu2     n/a                 default,performance,balance_performance,balance_power,power
cpu3     n/a                 default,performance,balance_performance,balance_power,power
cpu4     n/a                 default,performance,balance_performance,balance_power,power
cpu5     n/a                 default,performance,balance_performance,balance_power,power
cpu6     balance_performance default,performance,balance_performance,balance_power,power
cpu7     balance_performance default,performance,balance_performance,balance_power,power
cpu8     n/a                 default,performance,balance_performance,balance_power,power
cpu9     n/a                 default,performance,balance_performance,balance_power,power
cpu10    n/a                 default,performance,balance_performance,balance_power,power
cpu11    n/a                 default,performance,balance_performance,balance_power,power

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

CPU     Online  Cur         Min         Max         Min limit   Max limit   EPB
------- ------- ----------- ----------- ----------- ----------- ----------- ----
cpu0    n/a     900.0 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     7
cpu1    true    900.0 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     7
cpu2    false   800.2 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
cpu3    false   800.1 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
cpu4    false   800.0 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
cpu5    false   800.1 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
cpu6    true    898.9 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     7
cpu7    true    900.0 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     7
cpu8    false   800.1 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
cpu9    false   800.1 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
cpu10   false   800.0 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
cpu11   false   800.3 mhz   800.0 mhz   3.2 ghz     800.0 mhz   4.1 ghz     n/a
```
