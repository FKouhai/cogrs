use anyhow::Result;
use clap::Parser;
use cogrs::cli::Cli;
use cogrs::inventory::manager;
use cogrs::modules::{ModuleType, Modules};

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

    let hosts: Vec<String> = manager
        .filter_hosts(cli.pattern.as_str(), cli.limit.as_deref())?
        .iter()
        .map(|h| h.name.to_string())
        .collect();

    if cli.list_hosts {
        // ansible seems to ignore everything else if --list-hosts is specified?
        for host in hosts {
            println!("{host}");
        }
    } else if let Some(module_name) = cli.module_name {
        let modules = Modules::new();
        let module_type: ModuleType = module_name.parse()?;
        modules.run(module_type, cli.args.as_deref());
    } else {
        todo!()
    }

    Ok(())
}
