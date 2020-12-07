use std::io::{self, Read};
use ssvm_tensorflow_interface;

fn main() {
    let model_data: &[u8] = include_bytes!("lite-model_aiy_vision_classifier_birds_V1_3.tflite");
    let labels = include_str!("aiy_birds_V1_labels.txt");

    let mut img_buf = Vec::new();
    io::stdin().read_to_end(&mut img_buf).unwrap();
    let flat_img = ssvm_tensorflow_interface::load_jpg_image_to_rgb32f(&img_buf, 224, 224);

    let mut session = ssvm_tensorflow_interface::Session::new(&model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
    session.add_input("module/hub_input/images_uint8", &flat_img, &[1, 224, 224, 3])
           .run();
    let res_vec: Vec<u8> = session.get_output("module/prediction");

    let mut i = 0;
    let mut max_index: i32 = -1;
    let mut max_value: u8 = 0;
    while i < res_vec.len() {
        let cur = res_vec[i];
        if cur > max_value {
            max_value = cur;
            max_index = i as i32;
        }
        i += 1;
    }
    println!("{} : {}", max_index, max_value as f32 / 255.0);

    let mut confidence = "low";
    if max_value > 200 {
        confidence = "very high";
    } else if max_value > 125 {
        confidence = "high";
    } else if max_value > 80 {
        confidence = "medium";
    }

    let mut label_lines = labels.lines();
    for _i in 0..max_index {
      label_lines.next();
    }
    println!("{} : {}", label_lines.next().unwrap().to_string(), confidence.to_string());
}
