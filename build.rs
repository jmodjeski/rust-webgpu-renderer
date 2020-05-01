extern crate glsl_to_spirv;

use std::error::Error;
use std::fs;
use glsl_to_spirv::ShaderType;

fn main() -> Result<(), Box<Error>> {
    // println!("cargo:rerun-if-changed=src/engine/shaders");

    fs::create_dir_all("compiled_shaders")?;

    for entry in fs::read_dir("src/engine/shaders")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            let shader_type = in_path.extension().and_then(|ext| {
                match ext.to_string_lossy().as_ref() {
                    "vert" => Some(ShaderType::Vertex),
                    "frag" => Some(ShaderType::Fragment),
                    _ => None,
                }
            });

            if let Some(shader_type) = shader_type {
                use std::io::Read;

                let source = fs::read_to_string(&in_path)?;
                let mut compiled_file = glsl_to_spirv::compile(&source, shader_type)?;
                let mut compiled_bytes = Vec::new();
                compiled_file.read_to_end(&mut compiled_bytes)?;

                let out_path = format!("compiled_shaders/{}.spv", in_path.file_name().unwrap().to_string_lossy());
                fs::write(&out_path, &compiled_bytes)?;
            }
        }
    }

    Ok(())
}