
use handlebars::Handlebars;

pub fn run() {
    let mut reg = Handlebars::new();
    // render without register
    println!(
        "{}",
        reg.template_render("Hello {{name}}", &json!({"name": "Gian"}))
            .unwrap()
    );

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")
        .unwrap();
    println!("{}", reg.render("tpl_1", &json!({"name": "Giansalex"})).unwrap());
}