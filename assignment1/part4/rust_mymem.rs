// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.
#![allow(unused_imports)]
use alloc::boxed::Box;
use kernel::file::{flags, File, Operations, SeekFrom};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::miscdev;

use kernel::prelude::*;
use kernel::sync::smutex::Mutex;
use kernel::sync::Arc;
use kernel::sync::ArcBorrow;

module! {
    type: Mdriver,
    name: "mdriver",
    author: "Alex Schott",
    license: "GPL",
}

struct Mdriver {
    _dev: Pin<Box<miscdev::Registration<Mdriver>>>,
}

struct Device {
    contents: Mutex<Vec<u8>>,
    cursor: Mutex<usize>,
}

impl kernel::Module for Mdriver {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust mdriver (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));
        let reg = miscdev::Registration::new_pinned(
            fmt!("mdriver"),
            Arc::try_new(Device {
                contents: Mutex::new(Vec::<u8>::new()),
                cursor: Mutex::new(0),
            })
            .unwrap(),
        )?;

        Ok(Self { _dev: reg })
    }
}

impl Drop for Mdriver {
    fn drop(&mut self) {
        pr_info!("Rust mdriver (exit)\n");
    }
}

#[vtable]
impl Operations for Mdriver {
    // let data = 10;
    type Data = Arc<Device>;
    type OpenData = Arc<Device>;

    fn open(ctxt: &Arc<Device>, _file: &File) -> Result<Arc<Device>> {
        // pr_info!("File open\n");

        Ok(ctxt.clone())
    }

    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        // pr_info!("File Read\n");
        // let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let mut cursor = data.cursor.lock();

        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(*cursor));
        // pr_info!("Cursor start - {} \n", *cursor);
        // writer.write_slice(&vec[*cursor..][..len])?;
        writer.write_slice(&vec[*cursor..][..len])?;

        *cursor += len;
        // pr_info!("Cursor end - {}, with read size {}\n", *cursor, len);

        Ok(len)
    }
    fn write(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        // pr_info!("File Write\n",);
        let mut cursor = data.cursor.lock();
        let len = reader.len();
        let len = len.checked_add(*cursor).ok_or(EINVAL)?;
        let mut vec = data.contents.lock();
        // pr_info!("Cursor {} \n", *cursor);

        if len + *cursor > vec.len() {
            vec.try_resize(len + *cursor, 0)?;
        }
        if len + *cursor > 512000 {
            pr_info!("File too large\n");
            return Err(EINVAL);
        }
        reader.read_slice(&mut vec[*cursor..][..len])?;
        *cursor += len;
        // pr_info!("Cursor end - {}, with write size {}\n", *cursor, len);

        // pr_info!("size of input\n", data.size);
        Ok(len)
    }
    fn seek(data: ArcBorrow<'_, Device>, _file: &File, offset: SeekFrom) -> Result<u64> {
        // pr_info!("File Seek\n");
        let mut cursor = data.cursor.lock();
        let _len = match offset {
            SeekFrom::Start(off) => {
                *cursor = off.try_into()?;
            }
            _ => {
                return Err(EINVAL);
            } // SeekFrom::End(off) => {
              //     *cursor = data
              //         .contents
              //         .lock()
              //         .len()
              //         .checked_add(off.try_into()?)
              //         .ok_or(EINVAL)?;
              // }
              // SeekFrom::Current(off) => {
              //     *cursor = cursor.checked_add(off.try_into()?).ok_or(EINVAL)?;
        };
        // pr_info!("Cursor end - {}\n", *cursor);
        Ok(*cursor as u64)
    }
}
