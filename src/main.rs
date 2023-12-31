mod models;

use crate::models::ip::Ip;

fn main() {
    let ip = Ip::new(
        String::from("target 1"),
        String::from("10.10.10.2"),
        String::from(""),
    );

    println!(
        r#"
Target details:
Name: {}
IP: {}
Comment: {}
"#,
        ip.get_name(),
        ip.get_ip(),
        ip.get_comment()
    );
}