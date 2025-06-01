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
  -cdrom iso/alpine-nyan-20250601-x86_64.iso
```

## Flash to USB

```sh
sudo dd bs=4M status=progress oflag=sync of=/dev/sdb if=iso/alpine-nyan-20250601-x86_64.iso
```

```sh
sudo eject /dev/sdb
```

## ttyX
- tty1 = nyan
- tty2 = alsamixer
- tty3 = getty
