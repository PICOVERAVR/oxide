use std::fs;
use std::collections::HashMap;
use toml::Value;
use oxide::ray::{Sphere, Material, Light, LightType};
use oxide::opts::*;
use oxide::vec::Vector;

pub fn read_cfg(path: &str) -> Option<(Config, Vec<Sphere>, Vec<Light>)> {
    let cfg_str = fs::read_to_string(path).unwrap();
    let cfg = cfg_str.parse::<Value>().unwrap();

    let cfg_base = cfg.try_into::<toml::map::Map<String, Value>>().unwrap();

    let mut c = Config { ..Default::default() }; // using struct update syntax to fill out field here

    let mut mats = HashMap::new();
    let mut objs = vec![];
    let mut lights = vec![];

    let get_v3 = |parent: &Value, s| -> Vector {
        let vals = parent[s].as_array().unwrap();
        Vector::from_3(vals[0].as_float().unwrap() as f32, vals[1].as_float().unwrap() as f32, vals[2].as_float().unwrap() as f32)
    };

    for (k, v) in cfg_base {
        match k.as_str() {
            "materials" => {
                let mat_map = v.try_into::<toml::map::Map<String, Value>>().unwrap();
                for (name, mv) in mat_map {
                    let color = get_v3(&mv, "color");
                    let spec = mv["spec"].as_float().unwrap() as f32;
                    let refl = mv["refl"].as_float().unwrap() as f32;
    
                    mats.insert(name, Material {color, spec, refl});
                }
            },
            "objects" => {
                let obj_map = v.try_into::<toml::map::Map<String, Value>>().unwrap();
                for (_, ov) in obj_map {
                    let c = get_v3(&ov, "center");
                    let r = ov["radius"].as_float().unwrap() as f32;
                    let mat = mats[ov["material"].as_str().unwrap()];

                    objs.push(Sphere {c, r, mat});
                }
            },
            "lights" => {
                let light_map = v.try_into::<toml::map::Map<String, Value>>().unwrap();
                for (_, lv) in light_map {
                    let kind = match lv["type"].as_str().unwrap() {
                        "ambient" => LightType::Ambient,
                        "directional" => LightType::Directional(get_v3(&lv, "position")),
                        "point" => LightType::Point(get_v3(&lv, "position")),
                        _ => panic!("found illegal light type!")
                    };
                    let color = get_v3(&lv, "color");

                    lights.push(Light {kind, color});
                }
            },
            "world" => c.world = World {
                cam_pos: get_v3(&v, "camera_position"),
            },
            "render" => c.render = Render {
                max_reflections: v["max_reflections"].as_integer().unwrap() as u32,
                threads: v["threads"].as_integer().unwrap() as u32,
            },
            "output" => c.output = Output {
                width: v["width"].as_integer().unwrap() as usize,
                height: v["height"].as_integer().unwrap() as usize,
                bits: v["bits"].as_integer().unwrap() as u32,
            },
            _ => panic!("unknown key!")
        }
    }

    Some((c, objs, lights))
}