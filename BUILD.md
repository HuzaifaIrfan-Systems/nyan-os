# Build

https://wiki.alpinelinux.org/wiki/How_to_make_a_custom_ISO_image_with_mkimage


```sh
git clone --depth=1 https://gitlab.alpinelinux.org/alpine/aports.git
```

```sh
mkdir workdir iso
docker compose up -d
docker compose exec alpine-builder sh
```

## Build ISO in Container

```sh
sh build.sh
```

[Home](README.md)