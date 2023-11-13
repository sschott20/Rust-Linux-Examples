#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{thread, time};
mod utils;

use nix;
use nix::unistd::Uid;

use nix::ioctl_write_ptr;
mod bindings;
use bindings::*;
use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::videoio::*;
use opencv::{highgui::*, prelude::*, videoio};
use std::net::TcpStream;

use memmap::Mmap;
use memmap::MmapOptions;
mod server;
mod setup;
use nix::sys::ioctl;
use nix::sys::select;
use nix::sys::select::FdSet;
use server::*;
use setup::*;
mod app;
use app::*;
use std::{fs::File, os::unix::prelude::AsRawFd, str};
use std::{
    fs::OpenOptions,
    io::{self, prelude::*, Seek, SeekFrom, Write},
};
use utils::*;

const IMG_SIZE: usize = 462848;

fn get_pfn(virtual_address: usize) -> io::Result<u64> {
    // println!("Virtual address: {}", virtual_address);
    let page_size = 4096; // Obtain this from sysconf(_SC_PAGESIZE) or page_size::get()
    let pagemap_entry_size = std::mem::size_of::<u64>();

    // Calculate the offset in the pagemap file for the corresponding virtual address
    let pagemap_offset = (virtual_address / page_size) * pagemap_entry_size;
    // to format as hex use {}
    // println!("Vaddr: {:x}, Offset: {:x}", virtual_address, pagemap_offset);

    // Open the pagemap file for the current process
    let mut pagemap_file = File::open("/proc/self/pagemap")?;

    // Seek to the corresponding entry in the pagemap file
    pagemap_file.seek(SeekFrom::Start(pagemap_offset as u64))?;

    // Read the pagemap entry
    let mut entry = [0; 8];
    pagemap_file.read_exact(&mut entry)?;

    // Convert to u64 and check if the page is present
    let entry_val = u64::from_ne_bytes(entry);
    // println!("Entry: {:#066b}", entry_val);
    let is_present = (entry_val >> 63) & 1 == 1;

    // Mask out the flags and shift to get the PFN if present
    let pfn = if is_present {
        entry_val & ((1 << 55) - 1)
    } else {
        println!("Page not present");
        0
    };

    Ok(pfn)
}

fn main() -> io::Result<()> {
    // let stream = TcpStream::connect("127.0.0.1:54321").expect("failed to connect");
    println!("User space client started\n");
    if !Uid::effective().is_root() {
        panic!("You must run this executable with root permissions");
    }
    let mut f = File::options().write(true).read(true).open("/dev/video2")?;

    let mut fd = f.as_raw_fd();

    // im sorry for this one and I hate borrow rules so much
    let mut ftmp = File::options()
        .write(true)
        .read(true)
        .create(true)
        .open("tmp")?;

    let mut client: App = App {
        buffer: unsafe { memmap::MmapOptions::new().len(1).map_mut(&ftmp)? },
        file: f,
        buf: unsafe { std::mem::zeroed() },
        media_fd: fd,
    };
    client.start_device(3);

    client.buffer = unsafe {
        memmap::MmapOptions::new()
            .len(client.buf.length as usize)
            .map_mut(&client.file)?
    };

    // let mut buffer: Vec<u8> = vec![0; 4096]; // This is your buffer
    // buffer[0] = 1;
    // let mut buffer: [u8; IMG_SIZE] = [69; IMG_SIZE];
    // let buffer_addr = buffer.as_ptr() as usize;

    let buffer_addr = client.buffer.as_ptr() as usize;
    // let pfn = get_pfn(buffer_addr)?;

    // open /dev/rust_client
    let mut f = File::options()
        .write(true)
        .read(true)
        .open("/dev/rust_client")?;

    let mut acc = 0;
    while acc < IMG_SIZE {
        let pfn = get_pfn(buffer_addr + acc)?;

        f.seek(SeekFrom::Start(pfn))?;
        acc = acc + 4096;
    }
    // loop {
    // let mut buffer = [0; 100];

    let mut buffer: Vec<u8> = vec![0; 110646];
    // let mut buffer: Vec<u8> = Vec::with_capacity(110646);
    // let mut tmp_buf: [u8; 110646] = [0; 110646];
    client.read();
    let len = f.read_exact(&mut buffer);
    // buffer.extend_from_slice(&tmp_buf);
    // let len = f.read_exact(&mut buffer);
    // print first 10 bytes from buffer:
    for i in 0..10 {
        println!("buffer: {:x}", buffer[i]);
    }
    // println!("buffer recieve size: {}", len.unwrap());
    client.qbuf();
    // f.read(&mut buffer)?;
    println!("buffer recieve size: {}", buffer.len());
    let mut flipped = Mat::default();

    opencv::imgcodecs::imdecode_to(
        &opencv::types::VectorOfu8::from_iter(buffer),
        -1,
        &mut flipped,
    )
    .unwrap();

    let frame = resize_with_padding(&flipped, [196 * 2, 196 * 2]);
    opencv::imgcodecs::imwrite("test.bmp", &frame, &Vector::new()).unwrap();
    // imshow("MoveNet", &frame).expect("imshow [ERROR]");

    // let key = wait_key(10000).unwrap();
    // if key > 0 && key != 255 {
    //     break;
    // }
    // }

    // now need to send that physical address to the kernel module

    // let kernel_addr = ::bindings::phys_to_virt(phy_addr);
    // println!("Kernel Address: {:x}", kernel_addr);
    Ok(())
    // find physical pages for the buffer
}
