mod config;

use std::fs::File;
use std::io::Write;
use std::{time, env};

// this call ensures that we're using the library version of the functions rather than including them in the binary and library
// if this is failing, make sure to run "cargo clean" if you built everything as a binary
use oxide::{render, mat, draw};

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: oxide <config file>");
        return Ok(())
    }

    let path = &args[1];

    let (cfg, spheres, lights) = config::read_cfg(path).expect("could not import config file");

    let w = cfg.output.width;
    let h = cfg.output.height;

    let header = format!("P6 {} {} {}\n", w, h, 2u32.pow(cfg.output.bits) - 1);

    let out_parts: Vec<&str> = path.split('.').collect();
    let out_path = String::from(out_parts[0]) + ".ppm";
    
    let mut file = File::create(out_path)?; // "?" unpacks the result if Ok and returns the error if not
    file.write_all(header.as_bytes())?;

    // print to stderr so output isn't buffered until the end
    eprintln!("\nrender dimensions: {} x {}", w, h);
    eprintln!("rendering... ");

    // split the render into vertical slices and divide amongst threads
    // (horizontal slices are harder to collapse together)

    // TODO: come back to threading after learning about Arc<T>?
    // since the closure we pass to thread::spawn has static lifetime

    let mut m_parts = vec![];

    let dt = (h / cfg.render.threads as usize) as i32;
    let start = -(h as i32) / 2 + dt / 2;
    
    let clock = time::Instant::now();

    m_parts.push(render::render(
        (0, start),
        (w, dt as usize),
        &spheres,
        &lights,
        &cfg
    ));

    m_parts.push(render::render(
        (0, start + dt),
        (w, dt as usize),
        &spheres,
        &lights,
        &cfg
    ));

    let time = clock.elapsed();
    
    eprintln!("done ({}.{} sec)\n", time.as_secs(), time.as_millis());    

    let get_bytes = |m: mat::Matrix<draw::Color>| -> Vec<u8> {
        let size = (m.rlen - 1) * (m.clen - 1) * 3;
        let mut buf: Vec<u8> = Vec::with_capacity(size);

        for y in 1..m.clen {
            for x in 0..m.rlen - 1 {
                let idx = x + y * (m.rlen - 1);
    
                let c = m.mat[idx];
    
                buf.push(c.r);
                buf.push(c.g);
                buf.push(c.b);
            }
        }

        buf
    };

    let mut bvec = vec![];

    for m in m_parts {
        bvec.append(&mut get_bytes(m));
    }

    assert_eq!(bvec.len(), w * h * 3);

    file.write_all(&bvec)?;

    Ok(())
}
