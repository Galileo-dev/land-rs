// FILE: build.rs
extern crate glob;
extern crate krpc_mars_terraformer;

fn main() {
    // Tell cargo to re-run this script only when json files in services/
    // have changed. You can choose to omit this step if you want to
    // re-generate services every time.
    for path in glob::glob("services/*.json")
        .unwrap()
        .filter_map(Result::ok)
    {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    krpc_mars_terraformer::run("services/", "src/").expect("Could not terraform Mars :(");
}
