#!/bin/sh -e

HOSTNAME="nyan"

cleanup() {
	rm -rf "$tmp"
}

makefile() {
	OWNER="$1"
	PERMS="$2"
	FILENAME="$3"
	cat > "$FILENAME"
	chown "$OWNER" "$FILENAME"
	chmod "$PERMS" "$FILENAME"
}

rc_add() {
	mkdir -p "$tmp"/etc/runlevels/"$2"
	ln -sf /etc/init.d/"$1" "$tmp"/etc/runlevels/"$2"/"$1"
}

tmp="$(mktemp -d)"
trap cleanup exit

mkdir -p "$tmp"/etc
mkdir -p "$tmp"/etc/apk
mkdir -p "$tmp"/etc/network
mkdir -p "$tmp"/root

cp ~/app/nyan "$tmp"/etc/


makefile root:root 0644 "$tmp"/etc/hostname <<EOF
$HOSTNAME
EOF


makefile root:root 0644 "$tmp"/etc/apk/world <<EOF
alpine-base
libgcc
gcompat
alsa-lib
alsa-utils
alsaconf
EOF


makefile root:root 0755 "$tmp"/etc/inittab <<EOF
# /etc/inittab

::sysinit:/sbin/openrc sysinit
::sysinit:/sbin/openrc boot
::wait:/sbin/openrc default

# tty1::respawn:/sbin/agetty 38400 tty1 --autologin root --noclear
tty1::respawn:/etc/nyan
tty2::respawn:/usr/bin/alsamixer
tty3::respawn:/sbin/getty 38400 tty3

::shutdown:/sbin/openrc shutdown

ttyS0::respawn:/sbin/getty -L 0 ttyS0 vt100
EOF

makefile root:root 0755 "$tmp"/etc/.xinitrc <<EOF
pulseaudio --daemon --system &
picom -c &
exec openbox-session
EOF

makefile root:root 0755 "$tmp"/etc/.profile <<EOF
#!/bin/sh -e

setup-devd -C mdev
startx
EOF

makefile root:root 0755 "$tmp"/etc/setup-script <<EOF
INTERFACESOPTS="auto lo
iface lo inet loopback

auto eth0
iface eth0 inet dhcp
    hostname saigo
"

KEYMAPOPTS="us us"
HOSTNAMEOPTS="-n saigo"
DNSOPTS="8.8.8.8"
TIMEZONEOPTS="-z UTC"
PROXYOPTS="none"
APKREPOSOPTS="-1"
SSHDOPTS="-c openssh"
NTPOPTS="-c openntpd"
DISKOPTS="-m sys /dev/vda"
EOF


makefile root:root 0755 "$tmp"/etc/setup.sh <<EOF
mkdir -p /root/.config

mv /etc/setup-alpine /sbin/setup-alpine
chmod +x /sbin/setup-alpine
cp /etc/.xinitrc /root/
cp /etc/.profile /root/
/root/.profile
EOF

makefile root:root 0644 "$tmp"/etc/motd <<EOF
Welcome to NYAN-OS!
EOF

rc_add devfs sysinit
rc_add dmesg sysinit
rc_add mdev sysinit
rc_add hwdrivers sysinit
rc_add modloop sysinit

rc_add hwclock boot
rc_add modules boot
rc_add sysctl boot
rc_add hostname boot
rc_add bootmisc boot
rc_add syslog boot

rc_add udev boot
rc_add dbus boot


rc_add libgcc boot
rc_add gcompat boot
rc_add alsa-lib boot

rc_add mount-ro shutdown
rc_add killprocs shutdown
rc_add savecache shutdown

tar -c -C "$tmp" etc | gzip -9n > $HOSTNAME.apkovl.tar.gz