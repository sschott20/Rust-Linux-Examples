set -e
rm 
cp /home/alex/assignment2/part4/kernelmod/rust_client.rs /home/alex/linux-cs429-fall-2023/samples/rust/ -f 
cd /home/alex/linux-cs429-fall-2023
make SUBDIRS=./samples/rust/ modules -j16
cp samples/rust/rust_client.ko ../cpsc429_schott/assignment2/part4/kernelmod/ -f 