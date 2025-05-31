FROM alpine:3.22

RUN apk add --no-cache alpine-sdk alpine-conf syslinux xorriso squashfs-tools grub grub-efi doas

RUN apk add --no-cache mtools dosfstools grub-efi nano

# RUN adduser -D -G abuild build

# Create 'build' user with an empty password and add to 'abuild'
RUN adduser -D -G abuild build && \
    echo "build::" | chpasswd -e

# Optional: allow passwordless doas access
RUN mkdir -p /etc/doas.d && \
    echo "permit nopass :abuild" > /etc/doas.d/doas.conf

USER build
WORKDIR /home/build

RUN abuild-keygen -i -a -n

RUN doas apk update

CMD ["sh"]
