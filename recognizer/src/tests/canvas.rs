use crate::canvas::Canvas;
use crate::tests::EX_01;

#[test]
fn development() {
    let mut canvas = Canvas::scan(&String::from(EX_01)).unwrap();

    // canvas.display_text_layer();
    // canvas.display_layer(LAYER_THIN);
    // canvas.display_layer(LAYER_BODY);
    // canvas.display_layer(LAYER_GRID);
    // canvas.display_layer(LAYER_REGIONS);
    // canvas.display_layer(LAYER_REGIONS_GRID);

    //canvas.display_all_layers();

    let _plane = canvas.plane().unwrap();
    // FIXME change to assert
    // println!("{}", plane);
}
