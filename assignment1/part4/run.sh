rmmod mdriver
make
mknod /dev/mdriver c 69 0
chown alex /dev/mdriver
insmod mdriver.ko
