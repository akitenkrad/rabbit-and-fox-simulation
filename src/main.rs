use charming::{
    component::{Axis, Title},
    element::{ItemStyle, Padding, Symbol, TextStyle},
    series::Scatter,
    theme::Theme,
    Chart, ImageFormat, ImageRenderer,
};

fn main() {
    let dt = 0.015 as f64;

    let br = 1.0;
    let df = 1.0;
    let a = 0.1;
    let b = 0.02;

    let mut f = 1 as f64;
    let mut r = 3 as f64;

    let mut sim_data_r = Vec::new();
    let mut sim_data_f = Vec::new();

    let mut t = 0 as f64;
    while t < 30.0 {
        t += dt;
        r = (1.0 + (br - a * f) * dt) * r;
        f = (1.0 + (b * r - df) * dt) * f;

        sim_data_r.push(vec![t, r]);
        sim_data_f.push(vec![t, f]);
    }

    plot(sim_data_r, sim_data_f, "output.png".to_string());
}

fn plot(sim_data_1: Vec<Vec<f64>>, sim_data_2: Vec<Vec<f64>>, out_path: String) {
    let chart = Chart::new()
        .title(
            Title::new()
                .text("Simulation Output")
                .text_style(TextStyle::new().font_size(32).font_weight("bold"))
                .padding(Padding::Double(10.0, 100.0)),
        )
        .x_axis(Axis::new())
        .y_axis(Axis::new())
        .series(
            Scatter::new()
                .symbol_size(4)
                .symbol(Symbol::Circle)
                .item_style(ItemStyle::new().opacity(0.5))
                .data(sim_data_1),
        )
        .series(
            Scatter::new()
                .symbol_size(4)
                .symbol(Symbol::Rect)
                .item_style(ItemStyle::new().opacity(0.5))
                .data(sim_data_2),
        );

    ImageRenderer::new(1080, 720)
        .theme(Theme::PurplePassion)
        .save_format(ImageFormat::Png, &chart, out_path)
        .expect("Failed to save image");
}
