import init, * as VoxelLib from "voxel-rust-lib";

init().then((module) => {
    VoxelLib.init();
    let f32_vec = VoxelLib.new_float32_vec();
    VoxelLib.float32_vec_write_test_data(f32_vec);
    let f32_vec_address = VoxelLib.float32_vec_data_address(f32_vec);
    let f32_vec_len = VoxelLib.float32_vec_len(f32_vec);
    let f32_vec_js = new Float32Array(module.memory.buffer, f32_vec_address, f32_vec_len);
    console.log(f32_vec_js);
});
