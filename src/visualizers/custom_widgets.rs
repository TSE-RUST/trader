// library dependencies
use druid::{Color, RenderContext, theme, Widget, WidgetExt};
use druid::widget::{Label, Painter};

// local dependencies
use crate::TraderUi;

/// function to create a custom button with color blue
///
/// **Federico Brancasi**
pub(crate) fn custom_button(name: &str) -> impl Widget<TraderUi> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rounded_rect(8.0);
        // let bounds = ctx.size().to_rect();


        // ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));
        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            // ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
            ctx.stroke(bounds, &Color::WHITE, 2.0);
        }

        if ctx.is_active() {
            // ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
            ctx.fill(bounds, &Color::rgb8(0, 128, 255));
        }
    });

    Label::new(format!("{}", name))
        .with_text_size(24.)
        .with_text_color(Color::BLACK)
        .center()
        .padding(5.0)
        .background(painter)
    // .expand()
}

/// function to create a custom button with color white
///
/// **Federico Brancasi**
pub(crate) fn custom_button_white(name: &str) -> impl Widget<TraderUi> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rounded_rect(8.0);

        ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));

        if ctx.is_disabled() && ctx.is_hot(){
            ctx.fill(bounds, &Color::rgb8(83, 83, 80));
            ctx.stroke(bounds, &Color::rgb8(41, 41, 41), 0.0);
        }

        if !ctx.is_disabled() && ctx.is_hot() {
            ctx.stroke(bounds, &Color::WHITE, 2.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0, 128, 255));
        }

        if ctx.is_disabled(){
            ctx.fill(bounds, &Color::rgb8(83, 83, 80));
        }

    });

    Label::new(format!("{}", name))
        .with_text_size(28.)
        .with_text_color(Color::BLACK)
        .center()
        .padding(10.0)
        .background(painter)
}

