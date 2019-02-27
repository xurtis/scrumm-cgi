//! Templates

use std::io::Write;
use cgi::{Response, html_response, http::{Uri, StatusCode}};
use horrorshow::*;

pub fn global_wrapper(title: String, content: impl Template) -> impl Template {
    let mut menu = Menu::new();

    menu.add_link("Test".to_owned(), Uri::from_static("http://google.com"));
    let dropdown = menu.child_menu("Test dropdown".to_owned());
    dropdown.add_link("First".to_owned(), Uri::from_static("/basic/path"));
    dropdown.add_divider();
    dropdown.add_link("Second".to_owned(), Uri::from_static("/basic/path"));

    html! {
        html {
            head {
                meta(charset = "utf-8");
                meta(
                    name = "viewport",
                    content = "width=device-width, initial-scale=1, shrink-to-fit=no"
                );
                title : title;
                : Stylesheet::from_resource(Resource::BOOTSTRAP_DARKLY);
            }
            body {
                : main_navbar(menu);
                : content;
                : Script::from_resource(Resource::JQUERY);
                : Script::from_resource(Resource::POPPER);
                : Script::from_resource(Resource::BOOTSTRAP_JS);
            }
        }
    }
}

pub fn main_navbar(menu: Menu) -> impl Template {
    fn nav_dropdown_item(item: MenuItem) -> Box<dyn RenderBox> {
        match item {
            MenuItem::Menu(name, menu) => box_html! {
            },
            MenuItem::Link(name, location) => box_html! {
                a (
                    class = labels!("dropdown-item"),
                    href = location.to_string()
                ) : name;
            },
            MenuItem::Divider => box_html! {
                div (class = labels!("dropdown-divider")) {}
            },
        }
    }

    fn nav_dropdown(name: String, menu: Menu) -> impl Template {
        let id: String = name.chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();
        let labelledby = id.clone();
        html! {
            li (
                class = labels!("nav-item", "dropdown")
            ) {
                a (
                    class = labels!("nav-link", "dropdown-toggle"),
                    href = "#",
                    id = id,
                    role = "button",
                    data-toggle = "dropdown",
                    aria-haspopup = "true",
                    aria-extended = "false"
                ) : name;
                div (
                    class = labels!("dropdown-menu"),
                    aria-labelledby = labelledby
                ) {
                    @for item in menu.0 {
                        : nav_dropdown_item(item);
                    }
                }
            }
        }
    }

    fn nav_menu_item(item: MenuItem) -> Box<dyn RenderBox> {
        match item {
            MenuItem::Menu(name, menu) => box_html! {
                : nav_dropdown(name, menu);
            },
            MenuItem::Link(name, location) => box_html! {
                li (class = labels!("nav-item")) {
                    a (
                        class = labels!("nav-link"),
                        href = location.to_string()
                    ) : name;
                }
            },
            MenuItem::Divider => box_html! {
            },
        }
    }

    fn nav_menu(menu: Menu) -> impl Template {
        html! {
            ul (
                class = labels!("navbar-nav mr-auto")
            ) {
                @for item in menu.0 {
                    : nav_menu_item(item);
                }
            }
        }
    }

    html! {
        nav(
            class = labels!("navbar", "navbar-expand-lg", "navbar-light", "bg-primary", "sticky-top")
        ) {
            a(
                class = labels!("navbar-brand"),
                href = "#"
            ): "Agile for GitHub";
            button(
                class = labels!("navbar-toggler"),
                type = "button",
                data-toggle = "collapse",
                data-target = "#mainNavbarContent",
                aria-controls = "mainNavbarContent",
                aria-expanded = "false",
                aria-label = "Toggle navigation"
            ) {
                span(class = "navbar-toggler-icon");
            }
            div(
                class = labels!("collapse", "navbar-collapse"),
                id = "mainNavbarContent"
            ) {
                : nav_menu(menu);
            }
        }
    }
}


pub struct Menu(Vec<MenuItem>);
enum MenuItem {
    Menu(String, Menu),
    Link(String, Uri),
    Divider,
}

impl Menu {
    fn new() -> Self {
        Menu(Vec::new())
    }

    fn add_link(&mut self, name: String, location: Uri) {
        self.0.push(MenuItem::Link(name, location));
    }

    fn add_divider(&mut self) {
        self.0.push(MenuItem::Divider);
    }

    fn child_menu(&mut self, name: String) -> &mut Menu {
        self.0.push(MenuItem::Menu(name, Menu::new()));
        if let Some(MenuItem::Menu(_, menu)) = self.0.last_mut() {
            menu
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Resource {
    uri: &'static str,
    integrity: &'static str,
}

impl Resource {
    const fn new(uri: &'static str, integrity: &'static str) -> Self {
        Resource { uri, integrity }
    }

    /// The stylesheet for the Darkly bootstrap theme from bootswatch.com
    const BOOTSTRAP_DARKLY: Resource = Self::new(
        "https://bootswatch.com/4/darkly/bootstrap.min.css",
        "sha384-w+8Gqjk9Cuo6XH9HKHG5t5I1VR4YBNdPt/29vwgfZR485eoEJZ8rJRbm3TR32P6k",
    );

    /// JQuery minified source
    const JQUERY: Resource = Self::new(
        "https://code.jquery.com/jquery-3.3.1.slim.min.js",
        "sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo",
    );

    const POPPER: Resource = Self::new(
        "https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.7/umd/popper.min.js",
        "sha384-UO2eT0CpHqdSJQ6hJty5KVphtPhzWj9WO1clHTMGa3JDZwrnQq4sF86dIHNDz0W1",
    );

    const BOOTSTRAP_JS: Resource = Self::new(
        "https://stackpath.bootstrapcdn.com/bootstrap/4.3.1/js/bootstrap.min.js",
        "sha384-JjSmVgyd0p3pXB1rRibZUAYoIIy6OrQ6VrjIEaFf/nJGzIxFDsf4x0xIM+B07jRM",
    );
}

#[derive(Debug)]
pub struct Stylesheet(Uri, &'static str);

impl Stylesheet {
    fn from_resource(resource: Resource) -> Self {
        Stylesheet(Uri::from_static(resource.uri), resource.integrity)
    }
}

impl Render for Stylesheet {
    fn render(&self, tmpl: &mut TemplateBuffer) {
        let template = html! {
            link(
                rel = "stylesheet",
                type = "text/css",
                href = self.0.to_string(),
                integrity = self.1,
                crossorigin = "anonymous"
            );
        };
        template.render(tmpl);
    }
}

impl RenderOnce for Stylesheet {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl RenderMut for Stylesheet {
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

#[derive(Debug)]
pub struct Script(Uri, &'static str);

impl Script {
    fn from_resource(resource: Resource) -> Self {
        Script(Uri::from_static(resource.uri), resource.integrity)
    }
}

impl Render for Script {
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl.write_raw(&format!(
            "<script
                type=\"application/javascript\"
                src=\"{}\"
                integrity=\"{}\"
                crossorigin=\"anonymous\">
            </script>",
            self.0.to_string(),
            self.1,
        ));
    }
}

impl RenderOnce for Script {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl RenderMut for Script {
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

/// Render a template as a final web page.
pub fn render_page(content: impl Template) -> Response {
    let mut response_text = "<!DOCTYPE html>\r\n\r\n".to_owned();
    content.write_to_fmt(&mut response_text);
    html_response(StatusCode::OK, response_text)
}
