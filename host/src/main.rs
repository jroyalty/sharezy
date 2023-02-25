use std::error::Error;
use std::time::Duration;

use log::debug;

use clap::{Parser, ValueEnum};

use ddc_hi::{Ddc, Display};

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(value_enum)]
    command: CliCommands,

    /// Initiates a scan for perpherials on all adapters.
    #[arg(long)]
    scan: bool,

    /// Shows a list of available adapters.
    #[arg(long)]
    show_adapters: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CliCommands {
    /// Show summary of all attached displays.
    ShowDisplays,
    /// Show the capabilites of all attached displays.
    ShowCaps,
    /// Show summary of all Bluetooh adapters.
    ShowAdapters,
    /// Scan devices visible to all Bluetooth adapters.
    ScanDevices,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    match &cli.command {
        CliCommands::ShowCaps => println!("Display"),
        _ => println!("nope"),
    }

    debug!("querying for displays...");

    for mut disp in Display::enumerate() {
        if let Err(e) = disp.update_capabilities() {
            println!("unable to get capabilities for display: {}", e);
            continue;
        }

        println!("{:?}", disp.info.mccs_database);

        // println!(
        //     "{:?} {}: {:?} {:?}",
        //     disp.info.backend,
        //     disp.info.id,
        //     disp.info.manufacturer_id,
        //     disp.info.model_name
        // );
        // if let Some(feature) = disp.info.mccs_database.get(0xdf) {
        //     let value = disp.handle.get_vcp_feature(feature.code).unwrap();
        //     println!("{}: {:?}", feature.name.as_ref().unwrap(), value);
        // }
    }

    // let runtime = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(2)
    //     .enable_all()
    //     .build()?;

    // println!("main is done");

    Ok(())
}

async fn do_with_manager(mgr: &Manager) {
    let adapter_list = mgr.adapters().await.unwrap();
    debug!("Found {} BT adapters", adapter_list.len());
}

async fn bags() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);
    println!("Hello world");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    // Parse the string of data into serde_json::Value.
    let v: serde_json::Value = serde_json::from_str(data).unwrap();

    config::Config { id: 123 };

    println!("{:?}", v["name"]);
}

// #[tokio::main(flavor = "current_thread")]
async fn _main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let manager = Manager::new().await?;
    do_with_manager(&manager).await;

    // let adapter_list = manager.adapters().await?;
    // debug!("Found {} BT adapters", adapter_list.len());
    // if adapter_list.is_empty() {
    // eprintln!("No Bluetooth adapters found");
    // }

    /*
    for adapter in adapter_list.iter() {
        println!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                println!(
                    "Peripheral {:?} is connected: {:?}",
                    local_name, is_connected
                );
                // if !is_connected {
                //     println!("Connecting to peripheral {:?}...", &local_name);
                //     if let Err(err) = peripheral.connect().await {
                //         eprintln!("Error connecting to peripheral, skipping: {}", err);
                //         continue;
                //     }
                // }
                // let is_connected = peripheral.is_connected().await?;
                // println!(
                //     "Now connected ({:?}) to peripheral {:?}...",
                //     is_connected, &local_name
                // );
                // peripheral.discover_services().await?;
                // println!("Discover peripheral {:?} services...", &local_name);
                // for service in peripheral.services() {
                //     println!(
                //         "Service UUID {}, primary: {}",
                //         service.uuid, service.primary
                //     );
                //     for characteristic in service.characteristics {
                //         println!("  {:?}", characteristic);
                //     }
                // }
                // if is_connected {
                //     println!("Disconnecting from peripheral {:?}...", &local_name);
                //     peripheral
                //         .disconnect()
                //         .await
                //         .expect("Error disconnecting from BLE peripheral");
                // }
            }
        }
    }
    */
    Ok(())
}
