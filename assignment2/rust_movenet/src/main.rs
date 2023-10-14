use opencv::core::{flip, Vec3b};
use opencv::videoio::*;
use opencv::{
	prelude::*,
	videoio,
	highgui::*,
};

mod utils;
use utils::*;
use tflitec::interpreter::{Interpreter, Options};
use tflitec::model::Model;

fn main() {
	// load model and create interpreter
	let options = Options::default();
	let path = format!("resource/lite-model_movenet_singlepose_lightning_tflite_int8_4.tflite");
	let model = Model::new(&path).expect("Load model [FAILED]");
	let interpreter = Interpreter::new(&model, Some(options)).expect("Create interpreter [FAILED]");
	interpreter.allocate_tensors().expect("Allocate tensors [FAILED]");
	// Resize input
	
	// open camera
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(); // 0 is the default camera
	videoio::VideoCapture::is_opened(&cam).expect("Open camera [FAILED]");
	cam.set(CAP_PROP_FPS, 30.0).expect("Set camera FPS [FAILED]");

	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame).expect("VideoCapture: read [FAILED]");

		if frame.size().unwrap().width > 0 {
			// flip the image horizontally
			let mut flipped = Mat::default();
			flip(&frame, &mut flipped, 1).expect("flip [FAILED]");
			// resize the image as a square, size is 
			let resized_img = resize_with_padding(&flipped, [192, 192]);

			// turn Mat into Vec<u8>
			let vec_2d: Vec<Vec<Vec3b>> = resized_img.to_vec_2d().unwrap();
			let vec_1d: Vec<u8> = vec_2d.iter().flat_map(|v| v.iter().flat_map(|w| w.as_slice())).cloned().collect();
			// set input (tensor0)
			interpreter.copy(&vec_1d[..], 0).unwrap();
			
			// run interpreter
			interpreter.invoke().expect("Invoke [FAILED]");

			// get output
			let output_tensor = interpreter.output(0).unwrap();
			draw_keypoints(&mut flipped, output_tensor.data::<f32>(), 0.25);
			imshow("MoveNet", &flipped).expect("imshow [ERROR]");
		}
		// keypress check
		let key = wait_key(1).unwrap();
		if key > 0 && key != 255 {
			break;
		}
	}
}