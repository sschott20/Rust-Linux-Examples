set -e
rm /home/alex/linux-cs429-fall-2023/samples/rust/rust_client.rs -f
cp /home/alex/cpsc429_schott/assignment2/part4/kernelmod/rust_client.rs /home/alex/linux-cs429-fall-2023/samples/rust/ -f 
cp /home/alex/cpsc429_schott/assignment2/part4/kernelmod/v4l2bindings.rs /home/alex/linux-cs429-fall-2023/samples/rust/ -f 
cp /home/alex/cpsc429_schott/assignment2/part4/kernelmod/ionum.rs /home/alex/linux-cs429-fall-2023/samples/rust/ -f 

cd /home/alex/linux-cs429-fall-2023
make SUBDIRS=./samples/rust/ modules -j16
cp /home/alex/linux-cs429-fall-2023/samples/rust/rust_client.ko ../cpsc429_schott/assignment2/part4/kernelmod/ -f 

sh /home/alex/cpsc429_schott/assignment2/part4/commit.sh