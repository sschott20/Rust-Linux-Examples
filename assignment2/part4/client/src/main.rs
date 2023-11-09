#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{thread, time};
mod utils;

use nix;
use nix::ioctl_write_ptr;
mod bindings;
use bindings::*;
use opencv::core::{flip, Mat, Vec3b};
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
use std::io::{self, prelude::*, SeekFrom};
mod app;
use app::*;
use std::{fs::File, os::unix::prelude::AsRawFd, str};
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};
use utils::*;

fn get_pfn(virtual_address: usize) -> io::Result<u64> {
    let page_size = 4096; // Obtain this from sysconf(_SC_PAGESIZE)
    let pagemap_entry_size = std::mem::size_of::<u64>();

    // Calculate the offset in the pagemap file for the corresponding virtual address
    let page_number = virtual_address / page_size;
    let pagemap_offset = page_number * pagemap_entry_size;

    // Open the pagemap file for the current process
    let mut pagemap_file = File::open("/proc/self/pagemap")?;

    // Seek to the corresponding entry in the pagemap file
    pagemap_file.seek(SeekFrom::Start(pagemap_offset as u64))?;

    // Read the pagemap entry
    let mut entry = [0; 8];
    pagemap_file.read_exact(&mut entry)?;

    // Extract the PFN (Page Frame Number) from the entry
    let pfn_mask: u64 = ((1 << 55) - 1) & !((1 << 8) - 1); // PFN mask without flags
    let pfn = u64::from_ne_bytes(entry) & pfn_mask;

    Ok(pfn)
}

fn main() -> io::Result<()> {
    let mut f = File::options().write(true).read(true).open("/dev/video0")?;

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

    let buffer_addr = client.buffer.as_ptr() as usize;
    let pfn = get_pfn(buffer_addr)?;
    println!("PFN: {}", pfn);

    // find physical pages for the buffer
}
