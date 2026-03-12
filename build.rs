use std::{env, fs, path::Path};

const UPLOT_VERSION: &str = "1.6.32";

const FILES: &[(&str, &str)] = &[
    ("uPlot.iife.min.js", "uplot.min.js"),
    ("uPlot.min.css", "uplot.min.css"),
];

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    for &(remote_name, local_name) in FILES {
        let dest = out_path.join(local_name);
        if dest.exists() {
            continue;
        }

        let url = format!("https://unpkg.com/uplot@{UPLOT_VERSION}/dist/{remote_name}");

        let mut body = ureq::get(&url).call().unwrap().into_body();
        let bytes = body.read_to_vec().unwrap();

        fs::write(&dest, bytes).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/uplot-init.js");
    println!("cargo:rerun-if-changed=assets/uplot-bars.js");
    println!("cargo:rerun-if-changed=assets/uplot-charts.css");
}
