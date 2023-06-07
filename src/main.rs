use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use sysbot_rs::SysBotClient;

#[derive(Parser)]
struct MmoDataDumper {
    #[arg(help = "IP of the switch you are connecting to")]
    ip: String,
    #[arg(
        long,
        short,
        help = "Port to connect to on the switch. Default is 6000",
        default_value_t = 6000
    )]
    port: u16,
    #[arg(
        short,
        long,
        help = "Combine mmo.bin and mo.bin into one file (combo.bin)."
    )]
    combined: bool,
}

#[tokio::main]
async fn main() {
    let dumper_args: MmoDataDumper = MmoDataDumper::parse();

    if let Ok(mut client) = SysBotClient::connect(&dumper_args.ip, dumper_args.port).await {
        if let Ok(mo_data) = client
            .pointer_peek(&[0x42BA6B0, 0x2B0, 0x58, 0x18, 0x20], 0x190)
            .await
        {
            if !dumper_args.combined {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("mo.bin");
                if let Ok(mut file) = file {
                    file.write_all(&mo_data[..(mo_data.len() - 1)]).unwrap();
                }
            }

            if let Ok(mmo_data) = client
                .pointer_peek(&[0x42BA6B0, 0x2B0, 0x58, 0x18, 0x1B0], 0x3980)
                .await
            {
                if !dumper_args.combined {
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open("mmo.bin");
                    if let Ok(mut file) = file {
                        file.write_all(&mmo_data[..(mmo_data.len() - 1)]).unwrap();
                    }
                } else {
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open("combo.bin");
                    if let Ok(mut file) = file {
                        file.write_all(&mo_data[..(mo_data.len() - 1)]).unwrap();
                        file.write_all(&mmo_data[..(mmo_data.len() - 1)]).unwrap();
                    }
                }
            } else {
                println!("Failed to get data from console!")
            }
        } else {
            println!("Failed to get data from console!")
        }
    } else {
        println!("Failed to connect to the switch!")
    }
}
