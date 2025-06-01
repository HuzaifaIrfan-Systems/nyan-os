#!/bin/sh -e

qemu-system-x86_64 \
  -m 1024 \
  -cpu host \
  -enable-kvm \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE_4M.fd \
  -drive if=pflash,format=raw,file=OVMF_VARS_4M.fd \
  -boot d \
  -nic user \
  -audiodev pa,id=snd0,out.frequency=44100,out.channels=2 \
  -device intel-hda -device hda-duplex,audiodev=snd0 \
  -cdrom iso/alpine-nyan-20250601-x86_64.iso