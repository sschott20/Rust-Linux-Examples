// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.
#![allow(unused_imports)]
use kernel::bindings;
use kernel::prelude::*;

use kernel::sync::{CondVar, Mutex};
use kernel::task::Task;
use kernel::Module;

use rust_mdriverf;

module! {
    type: MTest,
    name: "mtest",
    author: "Alex Schott",
    license: "GPL",
}

kernel::init_static_sync! {
    static COUNT: Mutex<u64> = 0;
    static COUNT_IS_ZERO: CondVar;
}

const N: u64 = 10;
const W: u64 = 1000;
const INIT_VAL: u64 = 69;

struct MTest;

fn read_to_u64() -> u64 {
    let mut buffer = [0u8; 8];
    let _ = rust_mdriverf::read(&mut buffer, 0);
    return u64::from_le_bytes(buffer);
}

fn write_u64(val: u64) {
    let buffer = val.to_le_bytes();
    let _ = rust_mdriverf::write(&buffer, 0);
}

impl kernel::Module for MTest {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Mtest init! \n");

        let sizes = [1, 64, 1000, 64000, 512000];
        for j in 0..5 {
            let size = sizes[j];
            let mut total_time = 0;
            for i in 0..10 {
                let mut buffer: Vec<u8> = Vec::try_with_capacity(size)?;
                buffer.try_resize(size, i as u8)?;
                let mut start_time = bindings::timespec64 {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                let mut end_time = bindings::timespec64 {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                unsafe { bindings::ktime_get_ts64(&mut start_time) };
                let _ = rust_mdriverf::read(&mut buffer, 0);
                unsafe { bindings::ktime_get_ts64(&mut end_time) };
                total_time += (end_time.tv_nsec - start_time.tv_nsec) as u64;
            }
            pr_info!(
                "average {:?} size read time: {:?} \n",
                size,
                total_time / 10
            );
        }
        for j in 0..5 {
            let size = sizes[j];
            let mut total_time = 0;
            for i in 0..10 {
                let mut buffer: Vec<u8> = Vec::try_with_capacity(size)?;
                buffer.try_resize(size, i as u8)?;
                let mut start_time = bindings::timespec64 {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                let mut end_time = bindings::timespec64 {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                unsafe { bindings::ktime_get_ts64(&mut start_time) };
                let _ = rust_mdriverf::write(&mut buffer, 0);
                unsafe { bindings::ktime_get_ts64(&mut end_time) };
                total_time += (end_time.tv_nsec - start_time.tv_nsec) as u64;
            }
            pr_info!(
                "average {:?} size write time: {:?} \n",
                size,
                total_time / 10
            );
        }
        write_u64(INIT_VAL as u64);
        // pr_info!("Mtest init! \n");
        *COUNT.lock() = W;
        for i in 0..W {
            Task::spawn(fmt!("test{i}"), move || {
                let mut guard = COUNT.lock();
                *guard -= 1;
                if *guard == 0 {
                    COUNT_IS_ZERO.notify_all();
                }
                drop(guard);

                for _ in 0..N {
                    let cur: u64 = read_to_u64();
                    write_u64(cur + 1);
                }
            })
            .unwrap();
        }

        let mut guard = COUNT.lock();
        while *guard != 0 {
            let _ = COUNT_IS_ZERO.wait(&mut guard);
        }
        pr_info!("Final: {}, target {}", read_to_u64(), INIT_VAL + N * W);

        Ok(MTest)
    }
}

impl Drop for MTest {
    fn drop(&mut self) {
        pr_info!("exiting mtest! \n");
    }
}
