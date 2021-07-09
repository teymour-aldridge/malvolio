#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
mod test {
    use malvolio::prelude::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg(test)]
    macro_rules! component_named_app_with_html {
        ($($html:tt)*) => {
            struct App {}
            impl Component for App {
                type Properties = ();
                type Message = ();
                fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                    Self {}
                }
                fn update(&mut self, _msg: Self::Message) -> bool {
                    false
                }
                fn change(&mut self, _props: Self::Properties) -> bool {
                    false
                }
                fn view(&self) -> Html {
                    $($html)*
                }
            }
        }
    }

    #[cfg(feature = "with_yew")]
    #[cfg(not(tarpaulin))]
    #[wasm_bindgen_test]
    fn test_links_render() {
        use yew::prelude::*;

        component_named_app_with_html!(A::default()
            .attribute(malvolio::prelude::Href::new("https://example.com"))
            .attribute(malvolio::prelude::Id::new("link"))
            .to_html());
        yew::initialize();

        let document = web_sys::window().unwrap().document().unwrap();
        let root = document
            .create_element("div")
            .expect("failed to create element");
        yew::App::<App>::new().mount(root.clone());
        let link = root
            .get_elements_by_tag_name("a")
            .named_item("link")
            .unwrap();
        assert_eq!(
            &link
                .attributes()
                .get_named_item("href")
                .expect("href not set")
                .value(),
            "https://example.com"
        );
    }

    #[cfg(feature = "with_yew")]
    #[cfg(not(tarpaulin))]
    #[wasm_bindgen_test]
    fn test_form_rendering() {
        use yew::prelude::*;

        component_named_app_with_html!(Form::new()
            .attribute(Method::Post)
            .child(
                Input::default()
                    .attribute(Type::Text)
                    .attribute(Id::new("input1"))
                    .attribute(Placeholder::new("Class name")),
            )
            .child(
                Input::default()
                    .attribute(Id::new("input2"))
                    .attribute(Type::Textarea)
                    .attribute(Placeholder::new("Add a description for this class here.")),
            )
            .child(
                Input::default()
                    .attribute(Id::new("input3"))
                    .attribute(Type::Submit)
                    .attribute(Value::new("Create class")),
            )
            .to_html());
        yew::initialize();

        let document = web_sys::window().unwrap().document().unwrap();
        let root = document
            .create_element("div")
            .expect("failed to create element");
        root.set_id("link");
        yew::App::<App>::new().mount(root.clone());
        let inputs = root.get_elements_by_tag_name("input");
        let input1 = inputs.named_item("input1").unwrap();
        assert_eq!(
            &input1
                .attributes()
                .get_named_item("type")
                .expect("type not set")
                .value(),
            "text"
        );
        let input2 = inputs.named_item("input2").unwrap();
        assert_eq!(
            &input2
                .attributes()
                .get_named_item("type")
                .expect("type not set")
                .value(),
            "textarea"
        );
        let input3 = inputs.named_item("input3").unwrap();
        assert_eq!(
            &input3
                .attributes()
                .get_named_item("type")
                .expect("type not set")
                .value(),
            "submit"
        );
    }

    #[cfg(feature = "with_yew")]
    #[cfg(not(tarpaulin))]
    #[wasm_bindgen_test]
    fn test_heading_rendering() {
        use yew::prelude::*;
        component_named_app_with_html!(Div::new()
            .child(
                H1::new("Heading 1")
                    .attribute(Id::new("heading1-1"))
                    .attribute(Class::from("some-class"))
            )
            .to_html());
        yew::initialize();
        let document = web_sys::window().unwrap().document().unwrap();
        let root = document
            .create_element("div")
            .expect("failed to create element");
        root.set_id("link");
        yew::App::<App>::new().mount(root.clone());
        let h1 = root.get_elements_by_tag_name("h1");
        let heading1_1 = h1.named_item("heading1-1").unwrap();
        assert_eq!(heading1_1.text_content().unwrap(), "Heading 1");
        assert_eq!(heading1_1.get_attribute("class").unwrap(), "some-class")
    }
}
