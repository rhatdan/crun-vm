# The crun-vm OCI Runtime

This is an **experimental** [OCI Runtime] that enables [Podman], [Docker], and
[Kubernetes] to run Virtual Machine (VM) images. The objective is to make
running VMs (in simple configurations) as easy as running containers, using
standard container tooling and without the need for in-depth knowledge on
virtualization technologies like libvirt or KubeVirt.

<p align="center">
  <img src="docs/example.gif" width="680" />
</p>

### Major features

  - Use it with **Podman**, **Docker**, or **Kubernetes**.
  - Launch VMs from VM **image files** present on the host or packaged into
    **container images**.
  - Control VM CPU and memory allocation.
  - Provide **cloud-init** and **Ignition** configurations to VMs.
  - **Mount directories** into VMs.
  - Pass **block devices** through to VMs.
  - Expose qcow2 files and other disk images to VMs as block devices.
  - Pass **vfio-pci** and **mediated vfio-pci** devices through to VMs.
  - **Forward ports** from the host to VMs.
  - **`podman exec`**/**`docker exec`**/**`kubectl exec`** into VMs.

### Documentation

  1. [Installing crun-vm](docs/1-installing.md)
  2. [Using crun-vm as a Podman or Docker runtime](docs/2-podman-docker.md)
  3. [Using crun-vm as a Kubernetes runtime](docs/3-kubernetes.md)
  4. [Internals](docs/4-internals.md)

### License

This project is released under the GPL 2.0 (or later) license. See
[LICENSE](LICENSE).

[Docker]: https://www.docker.com/
[Kubernetes]: https://kubernetes.io/
[Podman]: https://podman.io/
[OCI Runtime]: https://github.com/opencontainers/runtime-spec/blob/v1.1.0/spec.md
