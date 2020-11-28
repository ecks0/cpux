#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod cli;
pub mod cpu;
pub mod cpufreq;
pub mod intel_pstate;

pub(crate) mod pseudofs;
pub(crate) mod sysfs;
