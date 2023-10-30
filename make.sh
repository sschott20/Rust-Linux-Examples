set -e
cp assignment2/part2/client/rust_client.rs ../linux-cs429-fall-2023/samples/rust/ -f 
cd /home/alex/linux-cs429-fall-2023
make SUBDIRS=./samples/rust/ modules -j16
cp samples/rust/rust_client.ko ../cpsc429_schott/assignment2/part2/client/ -f 