<br />

<div align="center">
  <h1>NYAN OS</h1>
  <p><h3 align="center">Linux Boot to Nyan Cat 🚀</h3></p>
</div>


•
<hr>

## Demo Video

[![Demo Video](https://img.youtube.com/vi/j4AJb-qvTO4/0.jpg)](https://www.youtube.com/watch?v=j4AJb-qvTO4)


## Build Nyan Rust App

```sh
sh app.sh
```

## Build ISO

[BUILD](BUILD.md)


## Setup QEMU

```sh
sudo apt-get install qemu ovmf
cp /usr/share/OVMF/OVMF_CODE_4M.fd .
cp /usr/share/OVMF/OVMF_VARS_4M.fd .
```

## Run

```sh
sh run.sh
```

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



## 🤝🏻 &nbsp;Connect with Me

<p align="center">
<a href="https://www.huzaifairfan.com"><img src="https://img.shields.io/badge/-huzaifairfan.com-1aa260?style=flat&logo=Google-Chrome&logoColor=white"/></a>
<a href="https://www.linkedin.com/in/huzaifairfan/"><img src="https://img.shields.io/badge/-Huzaifa%20Irfan-0072b1?style=flat&logo=Linkedin&logoColor=white"/></a>
<a href="https://github.com/HuzaifaIrfan/"><img src="https://img.shields.io/badge/-Huzaifa%20Irfan-4078c0?style=flat&logo=Github&logoColor=white"/></a>
<a href="mailto:contact@huzaifairfan.com"><img src="https://img.shields.io/badge/-contact@huzaifairfan.com-c71610?style=flat&logo=Gmail&logoColor=white"/></a>
<a href="https://www.instagram.com/huzaifairfan2001/"><img src="https://img.shields.io/badge/-@huzaifairfan2001-cd486b?style=flat&logo=Instagram&logoColor=white"/></a>
<a href="https://www.facebook.com/huzaifairfan2001/"><img src="https://img.shields.io/badge/-@huzaifairfan2001-4267B2?style=flat&logo=Facebook&logoColor=white"/></a>
</p>

## License

Licensed under the MIT License, Copyright 2025 Huzaifa Irfan. [LICENSE](LICENSE)