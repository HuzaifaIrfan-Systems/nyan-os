# nyan-os

## Build

[BUILD](BUILD.md)


## Setup QEMU

```sh
sudo apt-get install qemu ovmf
cp /usr/share/OVMF/OVMF_CODE_4M.fd .
cp /usr/share/OVMF/OVMF_VARS_4M.fd .
```

## Run

```sh
qemu-system-x86_64 \
  -m 1024 \
  -cpu host \
  -enable-kvm \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE_4M.fd \
  -drive if=pflash,format=raw,file=OVMF_VARS_4M.fd \
  -boot d \
  -nic user \
  -cdrom iso/alpine-nas-250531-x86_64
```

