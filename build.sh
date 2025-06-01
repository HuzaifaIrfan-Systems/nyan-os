#!/bin/sh -e

cp scripts/* aports/scripts/
sh aports/scripts/mkimage.sh \
    --workdir ~/workdir \
	--outdir ~/iso \
	--arch x86_64 \
	--repository https://dl-cdn.alpinelinux.org/alpine/edge/main \
	--extra-repository https://dl-cdn.alpinelinux.org/alpine/edge/community \
    --tag 20250601 \
	--profile nyan