use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cogrs::cli::Cli;
use cogrs_core::adhoc::{AdHoc, AdHocOptions};
use cogrs_core::inventory::manager;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    run()
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let inventory = cli.inventory.as_deref();

    let mut manager = manager::InventoryManager::new();
    manager.parse_sources(inventory)?;

    if cli.list_hosts {
        let hosts = manager.filter_hosts(cli.pattern.as_str(), cli.limit.as_deref())?;
        // ansible seems to ignore everything else if --list-hosts is specified?
        for host in hosts {
            println!("{}", host.name);
        }
    } else if let Some(module_name) = cli.module_name {
        let args = cli
            .args
            .with_context(|| anyhow!("No argument passed to {module_name} module"))?;

        let options = AdHocOptions {
            forks: cli.forks,
            poll_interval: Some(cli.poll_interval),
            task_timeout: cli.task_timeout,
            async_val: cli.async_val,
            one_line: cli.one_line,
        };

        AdHoc::run(&module_name, &args, manager, &options)?;
    } else {
        todo!()
    }

    Ok(())
}
