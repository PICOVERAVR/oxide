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

    // split the render into vertical slices and divide amongst threads
    // (horizontal slices are harder to collapse together)

    let mut m_parts = vec![];
    let mut handles = vec![];

    // wrap shared objects in Arc so the last thread to use em also deletes em
    let cfg = Arc::new(cfg);
    let lights = Arc::new(lights);
    let objs = Arc::new(objs);

    // if the rendered image is small (or the machine is massive), clamp max # threads
    let min_threads = std::cmp::min(cfg.render.threads, h);

    // show correct number of threads used in total
    let disp_threads = if h % min_threads == 0 {
        min_threads
    } else {
        min_threads + 1
    };

    let dt = (h / min_threads as usize) as i32;
    let start = -(h as i32) / 2 + dt / 2;

    // print to stderr so output isn't buffered until the end
    eprintln!(
        "\nrender parameters: {} x {}, {} thread(s) ({} x {} pixels per thread)",
        w, h, disp_threads, w, dt
    );
    eprintln!("rendering... ");

    let clock = time::Instant::now();

    let mut curr_h: usize = 0;

    for i in 0..min_threads {
        // increase ref count of shared objects
        let objs_c = Arc::clone(&objs);
        let lights_c = Arc::clone(&lights);
        let cfg_c = Arc::clone(&cfg);

        // launch thread and store handle for later
        handles.push(thread::spawn(move || -> mat::Matrix<draw::Color> {
            render::render(
                (0, start + dt * i as i32), // midpoints of subsection
                (w, dt as usize),           // width and height of subsection
                &objs_c,
                &lights_c,
                &cfg_c,
            )
        }));

        curr_h += dt as usize;
    }

    if h % min_threads != 0 {
        // need an extra thread here to handle remaining work since we can't split work into an even number of rows per thread

        let objs_c = Arc::clone(&objs);
        let lights_c = Arc::clone(&lights);
        let cfg_c = Arc::clone(&cfg);

        let final_dt = h - curr_h;

        // launch thread and store handle for later
        handles.push(thread::spawn(move || -> mat::Matrix<draw::Color> {
            render::render(
                (0, (curr_h + final_dt / 2) as i32), // midpoints of subsection
                (w, final_dt),                       // width and height of subsection
                &objs_c,
                &lights_c,
                &cfg_c,
            )
        }));

        curr_h += final_dt as usize;
    }

    assert_eq!(curr_h, h);

    for h in handles {
        m_parts.push(h.join().expect("child thread panicked"));
    }

    let time = clock.elapsed();

    eprintln!("done ({}.{:03} sec)\n", time.as_secs(), time.as_millis());
    let get_bytes = |m: mat::Matrix<draw::Color>| -> Vec<u8> {
        let size = (m.rlen - 1) * (m.clen - 1) * 3;
        let mut buf: Vec<u8> = Vec::with_capacity(size);

        for y in 0..m.clen - 1 {
            // iterate through valid rows (stopping before we hit the last row)
            for x in 0..m.rlen - 1 {
                // iterate through all columns, skipping invalid (last) one
                let idx = x + y * (m.rlen - 1) + y;

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
