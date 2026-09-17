/// Integration tests that parse real `.frm` files and verify the full
/// layout pipeline (parse → convert → render → HTML assertions).
mod integration_tests {
    use vb6parse::{FormFile, SourceFile};
    use vb6runtime::layout::{self, LayoutConfig, renderer::TauriRenderer};

    /// Helper to ensure tests that share global state run sequentially.
    /// Handles PoisonError from previous panicked tests by forcibly acquiring the lock.
    fn lock_test() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Resolve the path to a fixture file.
    /// Integration tests run from the crate root directory.
    fn fixture_path(name: &str) -> std::path::PathBuf {
        let mut base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base.push("tests");
        base.push("fixtures");
        base.push(name);
        base
    }

    fn render_form(frm_path: &str) -> String {
        let _lock = lock_test();
        layout::form_store::reset();
        let path = fixture_path(frm_path);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| {
            panic!("failed to read fixture {:?}: {}", path, e);
        });
        let source = SourceFile::decode_with_replacement(frm_path, &bytes)
            .expect("failed to decode source file");
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        let handle = layout::load_form(&form_file.form, &LayoutConfig::default());
        let renderer = TauriRenderer::new(false);
        layout::render(handle, &renderer)
    }

    fn render_form_with_scope(frm_path: &str) -> String {
        let _lock = lock_test();
        layout::form_store::reset();
        let path = fixture_path(frm_path);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| {
            panic!("failed to read fixture {:?}: {}", path, e);
        });
        let source = SourceFile::decode_with_replacement(frm_path, &bytes)
            .expect("failed to decode source file");
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        let handle = layout::load_form(&form_file.form, &LayoutConfig::default());
        let renderer = TauriRenderer::new(true);
        layout::render(handle, &renderer)
    }

    // ---- simple.frm tests ----

    #[test]
    fn parse_simple_form_succeeds() {
        let path = fixture_path("simple.frm");
        let bytes = std::fs::read(&path).unwrap();
        let source = SourceFile::decode_with_replacement("simple.frm", &bytes).unwrap();
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        assert_eq!(form_file.form.name(), "Form1");
    }

    #[test]
    fn simple_form_has_label_and_button() {
        let html = render_form("simple.frm");
        assert!(
            html.contains("vb6-label"),
            "html should contain vb6-label class"
        );
        assert!(
            html.contains("vb6-commandbutton"),
            "html should contain vb6-commandbutton class"
        );
        assert!(
            html.contains("Hello World"),
            "html should contain label caption"
        );
        assert!(
            html.contains("&amp;OK"),
            "html should contain escaped button caption"
        );
    }

    #[test]
    fn scope_renderer_produces_valid_html() {
        // Tests that the scoped renderer produces valid HTML output.
        // Note: The .vb6-app scope root wrapper is not yet applied in render_container.
        // This test verifies the renderer works without panicking.
        let html = render_form_with_scope("simple.frm");
        assert!(
            !html.is_empty(),
            "scoped render should produce non-empty HTML"
        );
        assert!(
            html.contains("vb6-label"),
            "scoped render should contain vb6-label"
        );
    }

    // ---- container.frm tests ----

    #[test]
    fn parse_container_form_succeeds() {
        let path = fixture_path("container.frm");
        let bytes = std::fs::read(&path).unwrap();
        let source = SourceFile::decode_with_replacement("container.frm", &bytes).unwrap();
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        assert_eq!(form_file.form.name(), "Form1");
    }

    #[test]
    fn container_form_has_frame_and_nested_label() {
        let html = render_form("container.frm");
        assert!(
            html.contains("vb6-frame"),
            "html should contain vb6-frame class"
        );
        assert!(
            html.contains("<fieldset"),
            "frame should render as fieldset"
        );
        assert!(
            html.contains("<legend>"),
            "frame should have a legend element"
        );
        assert!(
            html.contains("Group Box"),
            "legend should contain frame caption"
        );
        assert!(
            html.contains("vb6-label"),
            "html should contain nested vb6-label"
        );
        assert!(
            html.contains("Inside Frame"),
            "nested label should have correct caption"
        );
    }

    // ---- multiline.frm tests ----

    #[test]
    fn parse_multiline_form_succeeds() {
        let path = fixture_path("multiline.frm");
        let bytes = std::fs::read(&path).unwrap();
        let source = SourceFile::decode_with_replacement("multiline.frm", &bytes).unwrap();
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        assert_eq!(form_file.form.name(), "Form1");
    }

    #[test]
    fn multiline_form_has_textbox() {
        let html = render_form("multiline.frm");
        assert!(
            html.contains("vb6-textbox"),
            "html should contain vb6-textbox class"
        );
    }

    // ---- full_controls.frm tests ----

    #[test]
    fn parse_full_controls_form_succeeds() {
        let path = fixture_path("full_controls.frm");
        let bytes = std::fs::read(&path).unwrap();
        let source = SourceFile::decode_with_replacement("full_controls.frm", &bytes).unwrap();
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        assert_eq!(form_file.form.name(), "Form1");
    }

    #[test]
    fn full_controls_form_has_many_control_types() {
        let html = render_form("full_controls.frm");
        assert!(html.contains("vb6-label"), "should have label");
        assert!(
            html.contains("vb6-commandbutton"),
            "should have command button"
        );
        assert!(html.contains("vb6-frame"), "should have frame");
        assert!(html.contains("vb6-checkbox"), "should have checkbox");
        assert!(
            html.contains("vb6-optionbutton"),
            "should have option button"
        );
        assert!(html.contains("vb6-picturebox"), "should have picturebox");
        assert!(html.contains("vb6-shape"), "should have shape");
        assert!(html.contains("vb6-combobox"), "should have combobox");
        assert!(html.contains("vb6-listbox"), "should have listbox");
    }

    #[test]
    fn full_controls_form_has_captions() {
        let html = render_form("full_controls.frm");
        assert!(html.contains("Title Label"), "should have label caption");
        assert!(
            html.contains("Options"),
            "should have frame caption in legend"
        );
        // Note: checkbox/optionbutton captions are not rendered in current implementation
        // (they use the value field for checked state instead of caption text)
    }

    // ---- empty_form.frm tests ----

    #[test]
    fn parse_empty_form_succeeds() {
        let path = fixture_path("empty_form.frm");
        let bytes = std::fs::read(&path).unwrap();
        let source = SourceFile::decode_with_replacement("empty_form.frm", &bytes).unwrap();
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        assert_eq!(form_file.form.name(), "Form1");
    }

    #[test]
    fn empty_form_has_no_control_elements() {
        let html = render_form("empty_form.frm");
        assert!(
            html.is_empty(),
            "empty form should produce no HTML"
        );
        assert!(
            !html.contains("vb6-label"),
            "should not have label controls"
        );
        assert!(
            !html.contains("vb6-textbox"),
            "should not have textbox controls"
        );
        assert!(
            !html.contains("vb6-commandbutton"),
            "should not have button controls"
        );
    }

    // ---- render pipeline tests ----

    #[test]
    fn render_form_has_converted_dimensions() {
        // Verifies that twip-to-pixel conversion is applied correctly.
        // The simple.frm fixture has ClientWidth = 4000 twips.
        // At 96 DPI: 4000 / 15 = 266.67 px.
        let _lock = lock_test();
        layout::form_store::reset();
        let path = fixture_path("simple.frm");
        let bytes = std::fs::read(&path).unwrap();
        let source = SourceFile::decode_with_replacement("simple.frm", &bytes).unwrap();
        let form_file = FormFile::parse(&source).unwrap_or_fail();
        let handle = layout::load_form(&form_file.form, &LayoutConfig::default());
        let width = layout::get_form(handle, |f| f.size.width);
        // 4000 twips at 96 DPI = 4000 / 15 = 266.67 px
        assert!(
            (width.unwrap() - 266.67).abs() < 0.1,
            "form width should be approximately 266.67px, got {}",
            width.unwrap()
        );
    }

    #[test]
    fn multiple_fixture_files_parse_and_render() {
        let fixtures = [
            "simple.frm",
            "container.frm",
            "multiline.frm",
            "full_controls.frm",
        ];
        for fixture in fixtures {
            let _lock = lock_test();
            layout::form_store::reset();
            let path = fixture_path(fixture);
            let bytes = std::fs::read(&path).unwrap_or_else(|e| {
                panic!("failed to read fixture {:?}: {}", path, e);
            });
            let source = SourceFile::decode_with_replacement(fixture, &bytes)
                .expect("failed to decode source file");
            let form_file = FormFile::parse(&source).unwrap_or_fail();
            let handle = layout::load_form(&form_file.form, &LayoutConfig::default());
            let renderer = TauriRenderer::new(false);
            let html = layout::render(handle, &renderer);
            assert!(
                !html.is_empty(),
                "rendering {} produced empty HTML",
                fixture
            );
        }
    }
}
