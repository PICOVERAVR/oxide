mod config;

use std::{env, time};

// this call ensures that we're using the library version of the functions rather than including them in the binary and library
// if this is failing, make sure to run "cargo clean" if you built everything as a binary
use oxide::output::*;
use oxide::{draw, mat, opts, render};

use std::sync::Arc;
use std::thread;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: oxide <config file>");
        return Ok(());
    }

    let path = &args[1];

    let (cfg, objs, lights) = config::read_cfg(path).expect("could not import config file");

    let w = cfg.output.width;
    let h = cfg.output.height;

    let out_parts: Vec<&str> = path.split('.').collect();

    // print to stderr so output isn't buffered until the end
    eprintln!(
        "\nrender dimensions: {} x {} across {} threads",
        w, h, cfg.render.threads
    );
    eprintln!("rendering... ");

    // split the render into vertical slices and divide amongst threads
    // (horizontal slices are harder to collapse together)

    let mut m_parts = vec![];
    let mut handles = vec![];

    let dt = (h / cfg.render.threads as usize) as i32;
    let start = -(h as i32) / 2 + dt / 2;
    let clock = time::Instant::now();

    // wrap shared objects in Arc so the last thread to use em also deletes em
    let cfg = Arc::new(cfg);
    let lights = Arc::new(lights);
    let objs = Arc::new(objs);

    for i in 0..cfg.render.threads {
        // increase ref count of shared objects
        let objs_c = Arc::clone(&objs);
        let lights_c = Arc::clone(&lights);
        let cfg_c = Arc::clone(&cfg);

        // launch thread and store handle for later
        handles.push(thread::spawn(move || -> mat::Matrix<draw::Color> {
            render::render(
                (0, start + dt * i as i32),
                (w, dt as usize),
                &objs_c,
                &lights_c,
                &cfg_c,
            )
        }));
    }

    for h in handles {
        m_parts.push(h.join().expect("child thread panicked"));
    }

    let time = clock.elapsed();

    eprintln!("done ({}.{:03} sec)\n", time.as_secs(), time.as_millis());
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

    let path_pre = String::from(out_parts[0]);

    match cfg.output.format {
        opts::Format::Ppm => {
            let mut out_img = PPM::new(path_pre + ".ppm", w, h, cfg.output.bits);
            out_img.write(&bvec);
        }
        opts::Format::Png => {
            let mut out_img = PNG::new(path_pre + ".png", w, h, cfg.output.bits);
            out_img.write(&bvec);
        }
    }

    Ok(())
}
