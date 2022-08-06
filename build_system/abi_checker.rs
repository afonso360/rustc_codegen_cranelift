use super::build_sysroot;
use super::config;
use super::rustc_info::get_wrapper_file_name;
use super::utils::{spawn_and_wait, spawn_and_wait_with_input};
use build_system::SysrootKind;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn run(
    channel: &str,
    sysroot_kind: SysrootKind,
    target_dir: &Path,
    cg_clif_build_dir: &Path,
    host_triple: &str,
    target_triple: &str,
) {
    eprintln!("Running abi-checker");

    let mut abi_checker_path = env::current_dir().unwrap();
    abi_checker_path.push("abi-checker");
    env::set_current_dir(abi_checker_path).unwrap();

    // let mut rustc_clif = self.root_dir.clone();
    // rustc_clif.push("build");
    // rustc_clif.push(get_wrapper_file_name("rustc-clif", "bin"));

    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    cmd.arg("--");
    // cmd.arg("--conventions");
    // cmd.arg("c,cdecl,fastcall,stdcall");
    // cmd.arg("--pairs");
    // cmd.arg("rustc_calls_cgclif,cgclif_calls_rustc,cgclif_calls_cc,cc_calls_cgclif");
    cmd.arg("--pairs");
    // TODO: These fail: rustc_calls_cgclif / cgclif_calls_cc / cc_calls_cgclif
    cmd.arg("cgclif_calls_rustc");
    cmd.arg("--add-rustc-codegen-backend");
    cmd.arg("cgclif:/mnt/c/Users/Afonso/CLionProjects/rustc_codegen_cranelift/build/lib/librustc_codegen_cranelift.so");

    spawn_and_wait(cmd);
}
