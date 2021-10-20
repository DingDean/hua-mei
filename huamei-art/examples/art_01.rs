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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let frames = app.elapsed_frames();
    if frames == 1 {
        draw.background()
            .color(srgba(213.0 / 255.0, 227.0 / 255.0, 212.0 / 255.0, 0.001));
    }

    if frames % 60 == 0 {
        for color in &model.tradition.colors {
            let x = random_range(-512.0, 512.0);
            let y = random_range(-512.0, 512.0);
            let r = random_range(1.0, 10.0);

            draw.ellipse().radius(r).x_y(x, y).color(Srgb::new(
                color.rgb[0] as f32 / 256.0,
                color.rgb[1] as f32 / 256.0,
                color.rgb[2] as f32 / 256.0,
            ));
        }
    } else {
        draw.rect().w_h(1024.0, 1024.0).x_y(0.0, 0.0).color(srgba(
            213.0 / 255.0,
            227.0 / 255.0,
            212.0 / 255.0,
            0.04,
        ));
    }

    draw.to_frame(app, &frame).unwrap();
}
