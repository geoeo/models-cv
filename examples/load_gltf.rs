use std::boxed::Box;
use std::error::Error as StdError;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path).expect("runtime error");
    } else {
        println!("usage: load-gltf <FILE>");
    }
}

fn run(path: &str) -> Result<(), Box<dyn StdError>> {
    let (document,buffers,_) = gltf::import(path)?;
    println!("{:#?}", document);
    let byte_offset = 23616;
    let byte_length = 141696;
    let byte_end = byte_offset+byte_length;
    let byte_pos = &buffers[0].0[byte_offset..byte_end];
    assert_eq!(byte_pos.len(),byte_length);
    Ok(())
}
