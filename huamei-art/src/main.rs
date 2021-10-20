use huamei::Tradition;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    tradition: Tradition,
}

fn model(app: &App) -> Model {
    let tradition = Tradition::new();

    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model { tradition }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}
