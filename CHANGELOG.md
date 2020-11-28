# v0.1.2

- Commnad-line interface changes:

  - Group cli arguments according to subsystem:

    - `--cpu-*` for cpu control arguments
    - `--freq-*` for cpufreq control arguments
    - `--pstate-*` for intel_pstate control arguments

  - Add arguments to control the summary output:

    - `--show-all / -a` - show all tables
    - `--show-cpu` - show the cpu table
    - `--show-freq` - show the cpufreq table
    - `--show-pstate` - show the intel_pstate table 

    If `--show-*` arguments are present, then the default tables will not be displayed, and
    `--quiet` is ignored.

  - Add more short names for common arguments.

  - Show CPUs which can only be online (e.g. `cpu0`) as online and not `n/a`.

  - When access to a sysfs file results in a file-not-found error:

    - if the target CPU exists, then return feature-not-available.
    - if the target CPU does not exist, then signal file-not-found error.
    
    This should more gracefully handle i/o errors when requested features are missing
    from the system.

  - The intel_pstate table is no longer shown by default, unless:

    - If no `--show-*` arguments are present, and `--pstate-*` control arguments are
      present, then show the pstate table by default, per `--quiet`.

- Rust changes:

  - Group functions according to subsystem.

  - Most functions now have a variant that performs basic i/o error-handling
    on behalf of the caller. Using `cpufreq` for example:

    - `fn cur_khz(cpu_id: u64) -> Result<Option<u64>>` returns `Ok(Some(false))`
      if the corresponding sysfs file is not found (i.e. the requested feature is not available).

    - `fn try_cur_khz(cpu_id: u64) -> Result<u64>` returns `Err(pseudofs::Error::IoNotFound(...))`
      if the corresponding sysfs file is not found.
  
  - Eliminate potential issues with conversions to KHz in `cli`.

# v0.1.1

- initial release
