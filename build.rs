use std::io::Write;

extern crate shaderc;

fn main() {
    // Create folder for compiled binaries if it doesn't already exist
    std::fs::create_dir_all("shaders/spv").unwrap();

    let compiler = shaderc::Compiler::new().unwrap();
    let options = shaderc::CompileOptions::new().unwrap();

    let vertex_source =
        std::fs::read_to_string("shaders/shader.vert").expect("Failed to read vertex shader.");
    let fragment_source =
        std::fs::read_to_string("shaders/shader.frag").expect("Failed to read fragment shader.");

    println!("Read vertex_source and fragment_source");

    let vertex_binary_result = compiler
        .compile_into_spirv(
            &vertex_source,
            shaderc::ShaderKind::Vertex,
            "shaders/shader.vert",
            "main",
            Some(&options),
        )
        .unwrap();

    // let text_result = compiler
    //     .compile_into_spirv_assembly(
    //         &vertex_source,
    //         shaderc::ShaderKind::Vertex,
    //         "shaders/shader.vert",
    //         "main",
    //         Some(&options),
    //     )
    //     .unwrap();

    // std::fs::write("shaders/spv/shader.vert.spv", text_result.as_text()).unwrap();
    // std::fs::write(
    //     "shaders/spv/shader.vert.spv",
    //     vertex_binary_result.as_text(),
    // )
    // .unwrap();

    let mut output = std::fs::File::create("shaders/spv/shader.vert.spv").unwrap();
    output
        .write_all(vertex_binary_result.as_binary_u8())
        .unwrap();
    println!("VBR written");

    let fragment_binary_result = compiler
        .compile_into_spirv(
            &fragment_source,
            shaderc::ShaderKind::Fragment,
            "shaders/shader.frag",
            "main",
            Some(&options),
        )
        .unwrap();

    let mut output = std::fs::File::create("shaders/spv/shader.frag.spv").unwrap();
    output
        .write_all(fragment_binary_result.as_binary_u8())
        .unwrap();
}
