qemu-system-x86_64 -enable-kvm -cpu host -m 1G -bios /usr/share/ovmf/x64/OVMF.4m.fd -cdrom mtrxos.iso -vga virtio -display sdl,gl=on -audiodev pa,id=snd0 -machine pc,pcspk-audiodev=snd0
