// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

use kernel::prelude::*;

module! {
    type: Client,
    name: "rust_client",
    author: "Rust for Linux Contributors",
    description: "Rust minimal sample",
    license: "GPL",
}

struct Client {
    numbers: Vec<i32>,
}

impl kernel::Module for Client {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust client (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));

        let mut numbers = Vec::new();
        numbers.try_push(72)?;
        numbers.try_push(108)?;
        numbers.try_push(200)?;

        Ok(Client { numbers })
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust minimal sample (exit)\n");
    }
}
