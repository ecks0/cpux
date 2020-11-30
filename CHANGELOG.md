# v0.1.5

- Command-line interface changes: 

  - Fix missing multiplier in frequency display calculations.
  
# v0.1.4

- Command-line interface changes:

  - Add `--i915` arguments for Intel GPUs:
  
    - `--i915` to display current values (displayed by default if detected)
    - `--i915-freq-min` to set minimum frequency
    - `--i915-freq-max` to set maximum frequency
    - `--i915-freq-boost` to set boost frequency

  - Rename `WAIT` argument to `REFRESH`.
  - Cleanup help text.
  - More consistent argument error handling.

- Rust changes:

  - Switch from `clap` to `structopt`.
  - Create types from utility functions in old cli and move to `utils` module.
  - Add `drm` helper module.
  - Add `i915` public module.

# v0.1.3

- Re-add bounds check on `WAIT` argument.

# v0.1.2

- Commnad-line interface changes:

  - Group cli arguments by feature set (subsystem):

    - `--cpu-*` for cpu control arguments
    - `--freq-*` for cpufreq control arguments
    - `--pstate-*` for intel_pstate control arguments

  - Add arguments to control table output:

    - `--cpu` to print the cpu table
    - `--freq` to print the cpufreq table
    - `--pstate` to print the intel_pstate table 

    If no table arguments are present, then tables are printed for features
    detected on the system, per `--quiet/-q`.

    If table arguments are present, then the default tables will not be displayed,
    and `--quiet/-q` is ignored.

  - Update short argument names.

  - Show CPUs which can only be online (e.g. `cpu0`) as online and not `n/a`.

  - When access to a sysfs file results in a file-not-found error:

    - if the target CPU exists, then return feature-not-available
    - if the target CPU does not exist, then signal file-not-found error
    
    ...which lets cpux differentiate between features not present on the system (not
    an error), and the user entering CPU IDs that don't exist on the system (error).

- Rust changes:

  - Group functions according to subsystem.

  - Most functions now have a variant that performs basic i/o error-handling
    on behalf of the caller. Using `cpufreq` for example:

    - `fn cur_khz(cpu_id: u64) -> Result<Option<u64>>` returns `Ok(None)`
      if the corresponding sysfs file is not found, but `cpu_id` exists on the system (i.e. the
      CPU exists, but the requested feature is not available).
      

    - `fn try_cur_khz(cpu_id: u64) -> Result<u64>` returns `Err(pseudofs::Error::IoNotFound(...))`
      if the corresponding sysfs file is not found.
  
  - Eliminate potential issues with conversions to KHz in `cli`.

# v0.1.1

- initial release
