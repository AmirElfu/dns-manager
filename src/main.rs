/* This is AmirElfu. give me a kiss <3 */


use std::{net::Ipv4Addr, path::Path};
use cursive::{theme::Theme, views::{Dialog, SelectView}};
use nix::libc;
struct DNS {
    electro: Vec<Ipv4Addr>,
    shecan: Vec<Ipv4Addr>,
    radar_game : Vec<Ipv4Addr>,
    d403 : Vec<Ipv4Addr>,
    google : Vec<Ipv4Addr>,
    cloudflare : Vec<Ipv4Addr>,
    quad9 : Vec<Ipv4Addr>
}
// I HATE "E".
static PATH_LINUX_FILE: &str = "/etc/resolv.conf";
fn check_user_is_root /* haha :) */() {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("RUN US WITH ROOOOOOOOOOOOOOOOOT!");
        std::process::exit(1);
    }
}

fn main() {
    //check root :)
    check_user_is_root();
    let path1 = Path::new(PATH_LINUX_FILE);
    let mut siv = cursive::default();
    let default_theme = Theme::terminal_default();
    siv.set_theme(default_theme);
    let dns = DNS {
        electro: vec![Ipv4Addr::new(78, 157, 42, 100), Ipv4Addr::new(78, 157, 42, 101)],
        shecan: vec![Ipv4Addr::new(178, 22, 122, 100), Ipv4Addr::new(185, 51, 200, 2)],
        radar_game: vec![Ipv4Addr::new(10, 202, 10, 10), Ipv4Addr::new(10, 202, 10, 11)],
        d403: vec![Ipv4Addr::new(10, 202, 10, 202), Ipv4Addr::new(10, 202, 10, 102)],
        google: vec![Ipv4Addr::new(8, 8, 8, 8), Ipv4Addr::new(8, 8, 4, 4)],
        cloudflare: vec![Ipv4Addr::new(1, 1, 1, 1), Ipv4Addr::new(1, 0, 0, 1)],
        quad9: vec![Ipv4Addr::new(9, 9, 9, 9), Ipv4Addr::new(149, 112, 112, 112)],
    };
    let select = SelectView::new()
        .item("Electro", (dns.electro[0], dns.electro[1]))
        .item("Shecan", (dns.shecan[0], dns.shecan[1]))
        .item("Radar Game", (dns.radar_game[0], dns.radar_game[1]))
        .item("403 Online", (dns.d403[0], dns.d403[1]))
        .item("Google", (dns.google[0], dns.google[1]))
        .item("Cloudflare", (dns.cloudflare[0], dns.cloudflare[1]))
        .item("Quad9", (dns.quad9[0], dns.quad9[1]))
        .on_submit(move |s, &(dns1, dns2)| {
            change_dns_linux(&path1, dns1, dns2);
            s.add_layer(Dialog::info("DNS changed successfully!"));
        });

    siv.add_layer(Dialog::around(select).title("DNS Manager"));

    siv.run();
}

fn change_dns_linux(path: &Path, dns1: Ipv4Addr, dns2: Ipv4Addr) {
    use std::fs::OpenOptions;
    use std::io::Write;

    // Open the file in write mode NOTE : fs::open is for opening file in read only mode.
    let mut file = match OpenOptions::new().write(true).truncate(true).open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let content = format!("nameserver {}\nnameserver {}\n", dns1, dns2);

    if let Err(e) = file.write_all(content.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}