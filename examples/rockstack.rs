use fcsrv::onnx::Variant;

#[tokio::main]
async fn main() {
    let args = Default::default();

    let predictor = fcsrv::onnx::get_predictor(Variant::Rockstack, &args)
        .await
        .unwrap();

    let image_file = std::fs::read("images/rockstack/9258513.jpg").unwrap();
    let guess = predictor
        .predict(image::load_from_memory(&image_file).unwrap())
        .unwrap();
    assert_eq!(guess, 3);

    let image_file = std::fs::read("images/rockstack/50444558.jpg").unwrap();
    let guess = predictor
        .predict(image::load_from_memory(&image_file).unwrap())
        .unwrap();
    assert_eq!(guess, 2);
}
