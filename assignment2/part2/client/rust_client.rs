use kernel::prelude::*;

module! {
    type: Client,
    name: "client",
    author: "Alex Schott",
    license: "GPL",
}
struct Client;

impl kernel::Module for Client {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        // Print a banner to make sure our module is working
        pr_info!("------------------------\n");
        pr_info!("starting virtual device!\n");
        pr_info!("------------------------\n");
        Ok(Client)
    }
}
