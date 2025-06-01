# Build

https://wiki.alpinelinux.org/wiki/How_to_make_a_custom_ISO_image_with_mkimage




```sh
git clone --depth=1 https://gitlab.alpinelinux.org/alpine/aports.git
```

```sh
docker compose up -d
```

## Configuration

```sh
export PROFILENAME=nas
cat << EOF > ~/aports/scripts/mkimg.$PROFILENAME.sh
profile_$PROFILENAME() {
        profile_standard
        kernel_cmdline="unionfs_size=512M console=tty0 console=ttyS0,115200"
        syslinux_serial="0 115200"
        kernel_addons="zfs"
        apks="\$apks iscsi-scst zfs-scripts zfs zfs-utils-py
                cciss_vol_status lvm2 mdadm mkinitfs mtools nfs-utils
                parted rsync sfdisk syslinux util-linux xfsprogs
                dosfstools ntfs-3g
                "
        local _k _a
        for _k in \$kernel_flavors; do
                apks="\$apks linux-\$_k"
                for _a in \$kernel_addons; do
                        apks="\$apks \$_a-\$_k"
                done
        done
        apks="\$apks linux-firmware"
        apkovl="aports/scripts/genapkovl-mkimgoverlay.sh"
}
EOF
```
```sh
nano ~/aports/scripts/mkimg.$PROFILENAME.sh
```

```sh
chmod +x ~/aports/scripts/mkimg.$PROFILENAME.sh
```
```sh
cp ~/aports/scripts/genapkovl-dhcp.sh ~/aports/scripts/genapkovl-mkimgoverlay.sh
```
```sh
nano ~/aports/scripts/genapkovl-mkimgoverlay.sh
```

```
mkdir -p "$tmp"/etc/apk
makefile root:root 0644 "$tmp"/etc/apk/world <<EOF
alpine-base
<apk1-service>
<apk2-service>
EOF
```
```
rc_add <apk1-service> boot
rc_add <apk2-service> boot
```

## Build

```sh
cp scripts/* aports/scripts/
sh aports/scripts/mkimage.sh \
        --workdir ~/workdir \
	--outdir ~/iso \
	--arch x86_64 \
	--repository https://dl-cdn.alpinelinux.org/alpine/edge/main \
        --tag 20250601 \
	--profile nyan
```
