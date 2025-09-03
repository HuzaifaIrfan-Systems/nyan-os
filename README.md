<div align="center">
  <h1>NYAN OS</h1>
  <p><h3 align="center">Linux Boot to Nyan Cat 🚀</h3></p>
</div>

<hr>

## 🎬 Demo

[▶️![Demo](https://img.youtube.com/vi/j4AJb-qvTO4/maxresdefault.jpg)](https://www.youtube.com/watch?v=j4AJb-qvTO4)



# 🛠️ Development

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

# 🚀 Usage


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



# 📝 Documentation

# 📚 References


# 🤝🏻 Connect with Me

[![GitHub](https://img.shields.io/badge/Github-%23222.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/HuzaifaIrfan/)
[![Website](https://img.shields.io/badge/Website-%23222.svg?style=for-the-badge&logo=google-chrome&logoColor==%234285F4)](https://www.huzaifairfan.com)

# 📜 License

Licensed under the GPL3 License, Copyright 2025 Huzaifa Irfan. [LICENSE](LICENSE)
