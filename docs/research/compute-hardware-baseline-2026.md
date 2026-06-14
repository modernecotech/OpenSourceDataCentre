# Compute Hardware Baseline for Open Linux Sites

Last reviewed: 2026-06-14.

This project should standardize around a small number of cheap, repeatable hardware profiles instead of trying to support every board and GPU on day one. The baseline favors open-source Linux drivers, low power, low replacement cost, and enough performance for useful tenant workloads.

## Decision

Use three baseline compute lanes:

| Lane | Default hardware | Role | Why |
| --- | --- | --- | --- |
| `sbc.rk3588.control` | Radxa ROCK 5B+ or equivalent RK3588 board with 16-32 GB RAM, NVMe, 2.5GbE, and 48 VDC-to-USB-C/PoE power | Facility gateways, telemetry collectors, small tenant edge instances, local management services, offline recovery console | Low power, fast enough ARM cores, NVMe, good I/O, Debian/Armbian path, and open hardware/software documentation. |
| `gpu.rocm.16g.default` | AMD Radeon RX 9060 XT 16GB on commodity x86 host | Default GPU inference, media, OpenCL/HIP compute, modest AI fine-tuning, graphics/visualization | Good price/performance, 16 GB VRAM at the low-cost tier, open Linux graphics stack, and ROCm support for RDNA4-class Radeon cards. |
| `gpu.xpu.16g.lowpower` | Intel Arc Pro B50 16GB on commodity x86 host | Low-power GPU tenancy, media, visualization, OpenVINO/oneAPI jobs, small inference | 16 GB VRAM, 70 W board power, no external PCIe power, open upstream Linux driver path, and strong fit for power-constrained racks. |

The default tenant hardware offer should be:

- `edge-arm.small`: one RK3588 SBC slice, Debian/Armbian Linux, local NVMe, 2.5GbE.
- `edge-arm.cluster`: 3-5 RK3588 nodes with k3s or Incus for small sovereign edge services.
- `gpu-open.1x16g`: one AMD RX 9060 XT 16GB GPU with ROCm/HIP and Vulkan/OpenCL.
- `gpu-lowpower.1x16g`: one Intel Arc Pro B50 16GB GPU with Level Zero/OpenCL/OpenVINO.
- `cpu-standard`: ordinary x86 or ARM servers for VMs, Kubernetes, Ceph, databases, and control-plane services.

## Why This Mix

SBCs should not replace ordinary servers for storage-heavy or memory-heavy workloads. Their job is to make the datacentre cheaper to operate at the edge: management gateways, field protocol adapters, small tenant services, sensor processing, light AI, and resilient local control when the main cluster is degraded.

The GPU choice is split deliberately:

- AMD RX 9060 XT 16GB is the default because ROCm and Mesa keep the Linux stack more open than CUDA-only procurement, and the card gives useful 16 GB VRAM without workstation pricing.
- Intel Arc Pro B50 is the low-power lane because it gives 16 GB VRAM at 70 W and uses Intel's open graphics and compute runtime path, but its AI software ecosystem is less universal than ROCm/CUDA.
- NVIDIA is not the baseline because the best AI tooling still depends heavily on proprietary CUDA and proprietary driver components. A site can add a paid compatibility pool later if tenants require it.

## Linux and Firmware Position

All nodes should run open-source Linux distributions:

- RK3588 SBCs: Debian or Armbian first; vendor images only for bring-up or hardware diagnostics.
- x86 GPU hosts: Debian, Ubuntu Server LTS, Fedora, Rocky, or AlmaLinux with recent kernel and Mesa.
- Kubernetes worker nodes: Talos Linux, Fedora CoreOS, Flatcar, or a minimal Debian/Ubuntu profile where hardware support requires it.
- Tenant images: Debian, Ubuntu, Fedora, Rocky, AlmaLinux, and container images from reproducible OCI builds.

Be honest about firmware: most modern SBCs, NICs, SSDs, and GPUs still require binary firmware blobs. The project requirement should be open-source operating systems, open-source orchestration, open-source drivers where available, documented firmware provenance, and no proprietary cloud control plane.

## Hardware Profiles

| Profile | Planning unit | Power design allowance | Planning price | Notes |
| --- | --- | ---: | ---: | --- |
| `sbc.rk3588.control` | 1 Radxa ROCK 5B+ class board, case, NVMe, 48 VDC power module | 30 W | $140-$260 | Use 3 nodes minimum for management quorum where possible. |
| `sbc.rk3588.edge` | 1 RK3588 board with 16-32 GB RAM and NVMe | 30 W | $120-$240 | Good for edge tenants, local data collection, and low-power services. |
| `gpu.rocm.16g.default` | 1 AMD RX 9060 XT 16GB card | 160 W | $350-$430 | Default open-driver GPU accelerator lane. |
| `gpu.xpu.16g.lowpower` | 1 Intel Arc Pro B50 16GB card | 70 W | $350-$450 | Best power budget fit for low-power GPU density and display/media workloads. |
| `gpu.rocm.scaleup` | AMD RX 9070-class 16GB card | 220-260 W | $500-$700 | Optional higher-throughput pool after ROCm validation. |

## Rack Pattern

For a 250 kW pilot:

- 5 RK3588 SBCs for facility gateway, telemetry buffering, local console, and out-of-band tools.
- 2 GPU pilot servers, each with 2 AMD RX 9060 XT 16GB cards.
- 1 low-power GPU server with 2 Intel Arc Pro B50 cards if media/visual desktop/low-power inference is a target.
- 20 commodity CPU servers for general compute.
- 4 Ceph storage nodes.
- All critical nodes powered from 48 VDC rack shelves where the PSU path exists, otherwise through a documented DC-to-AC exception until a DC PSU is sourced.

## Procurement Rules

1. Buy hardware that boots a current open-source Linux kernel without a vendor-only fork for normal operation.
2. Require at least one upstream or community Linux path before accepting a board into the baseline.
3. Require Redfish, OpenBMC, serial console, or documented remote recovery for every server class.
4. Require GPU reset and multi-GPU tests before offering tenant GPU instances.
5. Treat "AI TOPS" claims as marketing until tested with the actual model runtime.
6. Keep GPU cards below 200 W for the default tier unless a site has proven cooling and power density.
7. Prefer 16 GB VRAM minimum for user-facing GPU services in 2027+.

## Source Notes

- Radxa ROCK 5B+ lists RK3588, quad Cortex-A76 plus quad Cortex-A55 CPU, up to 32 GB LPDDR5, dual M.2, 2.5GbE, Wi-Fi 6, and PoE support with a HAT: https://radxa.com/products/rock5/5bp/
- Radxa documentation states ROCK 5B/5B+ supports Debian, Armbian, hardware design schematics, software source code, and RK3588 NPU up to 6 TOPS: https://docs.radxa.com/en/rock5/rock5b/getting-started/introduction
- Orange Pi 5 Plus is a strong RK3588 alternative with similar CPU class and dual 2.5GbE: https://www.orangepi.org/html/hardWare/computerAndMicrocontrollers/details/Orange-Pi-5-plus.html
- AMD ROCm documentation describes ROCm as an open-source platform for AMD Instinct and Radeon GPUs: https://rocm.docs.amd.com/
- Phoronix tested the RX 9060 XT 16GB on Linux with ROCm 6.4.1 and noted RDNA4 RX 9060/9070 support in that ROCm generation: https://www.phoronix.com/review/radeon-rx-9060-xt-rocm
- Phoronix reported Intel Arc Pro B50 as a 16 GB, 70 W, $349 workstation GPU with open-source Linux driver support and Ubuntu 25.04/Linux 6.14/Mesa 25 guidance: https://www.phoronix.com/review/intel-arc-pro-b50-linux
- Intel Compute Runtime is an open-source Level Zero and OpenCL driver for Intel graphics hardware: https://github.com/intel/compute-runtime
