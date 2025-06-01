profile_nyan() {
        profile_standard

        apks="$apks
        libgcc
        gcompat
        alsa-lib alsa-utils alsaconf
	"
        apkovl="aports/scripts/genapkovl-nyan.sh"
}

