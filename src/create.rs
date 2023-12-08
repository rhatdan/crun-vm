// SPDX-License-Identifier: GPL-3.0-only

use std::error::Error;
use std::io;

use crate::util::{crun, extract_runner_root_into, find_single_file_in_directory};

pub fn create(
    global_args: &liboci_cli::GlobalOpts,
    args: &liboci_cli::Create,
) -> Result<(), Box<dyn Error>> {
    let mut spec = libocispec::runtime::Spec::load(
        args.bundle
            .join("config.json")
            .to_str()
            .expect("path is utf-8"),
    )?;

    // find VM image

    let root = spec
        .root
        .as_ref()
        .expect("config.json includes configuration for the container's root filesystem");

    let vm_image_path = find_single_file_in_directory(args.bundle.join(&root.path).join("disk"))?;

    // prepare root filesystem for runner container

    let runner_root_path = args.bundle.join("crun-qemu-runner-root");
    extract_runner_root_into(&runner_root_path)?;

    // adjust config for runner container

    spec.root = Some(libocispec::runtime::Root {
        path: runner_root_path
            .to_str()
            .expect("path is utf-8")
            .to_string(),
        readonly: None,
    });

    let process = spec.process.as_mut().expect("process config");
    process.command_line = None;
    process.args = Some(
        [
            "qemu-system-x86_64",
            "--machine",
            "q35,accel=kvm",
            "--cpu",
            "host",
            "--smp",
            "cores=2",
            "-m",
            "2G",
            "--blockdev",
            "driver=file,node-name=boot,filename=/vm-image",
            "--device",
            "virtio-blk-pci-non-transitional,drive=boot,bootindex=1",
            "--nographic",
        ]
        .iter()
        .map(ToString::to_string)
        .collect(),
    );

    let linux = spec.linux.as_mut().expect("linux config");
    let devices = linux.devices.get_or_insert_with(Vec::new);
    devices.push(libocispec::runtime::LinuxDevice {
        file_mode: None,
        gid: None,
        major: Some(10),
        minor: Some(232),
        path: "/dev/kvm".to_string(),
        device_type: "char".to_string(),
        uid: None,
    });

    let mounts = spec.mounts.get_or_insert_with(Vec::new);
    mounts.push(libocispec::runtime::Mount {
        destination: "/vm-image".to_string(),
        gid_mappings: None,
        options: Some(vec!["bind".to_string(), "rprivate".to_string()]),
        source: Some(
            vm_image_path
                .canonicalize()?
                .to_str()
                .expect("path is utf-8")
                .to_string(),
        ),
        mount_type: Some("bind".to_string()),
        uid_mappings: None,
    });

    spec.save(
        args.bundle
            .join("config.json")
            .to_str()
            .expect("path is utf-8"),
    )?;

    // create runner container

    crun_create(global_args, args)?;

    Ok(())
}

fn crun_create(global_args: &liboci_cli::GlobalOpts, args: &liboci_cli::Create) -> io::Result<()> {
    // build crun argument list

    let mut arg_list = Vec::new();

    if global_args.debug {
        arg_list.push("--debug");
    }

    if let Some(path) = &global_args.log {
        arg_list.push("--log");
        arg_list.push(path.to_str().expect("path is utf-8"));
    }

    if let Some(format) = &global_args.log_format {
        arg_list.push("--log-format");
        arg_list.push(format);
    }

    if args.no_pivot {
        arg_list.push("--no-pivot");
    }

    if let Some(path) = &global_args.root {
        arg_list.push("--root");
        arg_list.push(path.to_str().expect("path is utf-8"));
    }

    if global_args.systemd_cgroup {
        arg_list.push("--systemd-cgroup");
    }

    arg_list.push("create");

    arg_list.push("--bundle");
    arg_list.push(args.bundle.to_str().expect("path is utf-8"));

    if let Some(path) = &args.console_socket {
        arg_list.push("--console-socket");
        arg_list.push(path.to_str().expect("path is utf-8"));
    }

    if args.no_new_keyring {
        arg_list.push("--no-new-keyring");
    }

    arg_list.push("--preserve-fds");
    let preserve_fds = args.preserve_fds.to_string();
    arg_list.push(&preserve_fds);

    if let Some(path) = &args.pid_file {
        arg_list.push("--pid-file");
        arg_list.push(path.to_str().expect("path is utf-8"));
    }

    arg_list.push(&args.container_id);

    // run crun

    crun(arg_list)
}