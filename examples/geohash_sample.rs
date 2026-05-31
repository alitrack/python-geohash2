#[allow(dead_code)]
#[path = "../src/geohash_core.rs"]
mod geohash_core;

fn main() {
    let hashcode = match geohash_core::encode(35.0, 135.0) {
        Ok(value) => value,
        Err(error) => {
            println!("error with {error}");
            return;
        }
    };
    let hashcode = &hashcode[..19];
    println!("{hashcode}");

    match geohash_core::decode(hashcode) {
        Ok((latitude, longitude, _, _)) => println!("{latitude:.6} {longitude:.6}"),
        Err(error) => println!("error with {error}"),
    }
}
