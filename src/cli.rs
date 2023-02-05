use clap::Parser;

/// Get list of subdomain names for an address from crt.sh website
#[derive(Parser)]
#[command(author = "Szymon Błaszczyński <museyoucoulduse@gmail.com>", version, about = "get list of subdomains for an address from crt.sh website", long_about = None)]
pub struct Cli {
    /// set domain name
    pub domain: Option<String>,

    /// set output file
    #[arg(short, long, value_name = "FILE")]
    pub save: Option<String>,

}

