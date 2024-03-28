use std::{fs::File, path::Path};

use feature_generator::generation::{
    gen_context::{AppCtx, Config, GenCtx},
    Feature, Generator, Subfeature,
};

fn setup_application() {
    let path = Path::new("app/src/main/java/test/base/package/testapp");
    std::fs::create_dir_all(path).unwrap();
    std::fs::write(
        path.join("Application.kt"),
        include_str!("./mock/Application.kt"),
    )
    .unwrap()
}

fn setup_settings() {
    let path = Path::new("settings.gradle.kts");
    std::fs::write(path, include_str!("./mock/settings.gradle.kts")).unwrap();
}

fn setup_app_modules() {
    let path = Path::new("buildSrc/src/main/kotlin/app-modules.kt");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, include_str!("./mock/app-modules.kts")).unwrap();
}

fn setup_app_build_gradle() {
    let path = Path::new("app/build.gradle.kts");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, include_str!("./mock/app-build-gradle.kts")).unwrap();
}

fn teardown_application() {
    std::fs::remove_dir_all(Path::new("app")).unwrap();
}

fn teardown_settings() {
    std::fs::remove_file(Path::new("settings.gradle.kts")).unwrap();
}

fn teardown_app_modules() {
    let path = Path::new("buildSrc");
    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn test_generate_feature() {
    setup_application();
    setup_settings();
    setup_app_modules();
    setup_app_build_gradle();
    let feature = Feature {
        app_context: default_app_ctx(),
        feature_name: "home".into(),
    };
    let feature = GenCtx::from(feature);
    let generator = Generator::new(feature);
    generator.generate();

    assert_content_eq(
        Path::new("feature/home/api/src/main/kotlin/test/base/package/HomeFeatureEntry.kt"),
        include_str!("./exp/feature_api/HomeFeatureEntry.kt"),
    );

    assert_content_eq(
        Path::new("feature/home/api/build.gradle.kts"),
        include_str!("./exp/feature_api/build.gradle.kts"),
    );

    assert_content_eq(
        Path::new("feature/home/impl/build.gradle.kts"),
        include_str!("./exp/feature_impl/build.gradle.kts"),
    );

    assert_content_eq(
        Path::new("feature/home/impl/src/main/kotlin/test/base/package/root/HomeFeatureRoot.kt"),
        include_str!("./exp/feature_impl/FeatureRoot.kt"),
    );

    assert_content_eq(
        Path::new(
            "feature/home/impl/src/main/kotlin/test/base/package/subfeature/home/HomeSubfeature.kt",
        ),
        include_str!("./exp/feature_impl/Subfeature.kt"),
    );

    assert_content_eq(
        Path::new(
            "feature/home/impl/src/main/kotlin/test/base/package/subfeature/home/screen/HomeScreen.kt",
        ),
        include_str!("./exp/feature_impl/Screen.kt"),
    );

    assert_content_eq(
        Path::new(
            "feature/home/impl/src/main/kotlin/test/base/package/subfeature/home/screen/HomeScreenViewModel.kt",
        ),
        include_str!("./exp/feature_impl/ScreenViewModel.kt"),
    );

    assert_content_eq(
        Path::new("app/src/main/java/test/base/package/testapp/Application.kt"),
        include_str!("./exp/app/Application.kt"),
    );

    assert_content_eq(
        Path::new("settings.gradle.kts"),
        include_str!("./exp/settings.gradle.kts"),
    );

    assert_content_eq(
        Path::new("buildSrc/src/main/kotlin/app-modules.kt"),
        include_str!("./exp/app-modules.kts"),
    );

    assert_content_eq(
        Path::new("app/build.gradle.kts"),
        include_str!("./exp/app-build-gradle.kts"),
    );

    std::fs::remove_dir_all("feature").unwrap();
    teardown_application();
    teardown_settings();
    teardown_app_modules();
}

#[test]
fn generate_subfeature() {
    let feature = Subfeature {
        app_context: default_app_ctx(),
        feature_name: "home".into(),
        subfeature_name: "home-details".into(),
    };
    let feature = GenCtx::from(feature);
    let generator = Generator::new(feature);
    generator.generate();

    assert_content_eq(
        Path::new("feature/home/impl/src/main/kotlin/test/base/package/subfeature/homedetails/HomeDetailsSubfeature.kt"),
        include_str!("./exp/feature_impl/subfeature_gen/Subfeature.kt"),
    );

    assert_content_eq(
        Path::new("feature/home/impl/src/main/kotlin/test/base/package/subfeature/homedetails/screen/HomeDetailsScreen.kt"),
        include_str!("./exp/feature_impl/subfeature_gen/Screen.kt"),
    );

    assert_content_eq(
        Path::new("feature/home/impl/src/main/kotlin/test/base/package/subfeature/homedetails/screen/HomeDetailsScreenViewModel.kt"),
        include_str!("./exp/feature_impl/subfeature_gen/ScreenViewModel.kt"),
    );
}

#[test]
fn generate_config() {
    let config = Config {
        app_name: Some("my-app".to_string()),
        base_package_name: Some("com.my.app".to_string()),
        global: false,
    };

    let ctx = GenCtx::from(config);
    let generator = Generator::new(ctx);
    generator.generate();

    assert_content_eq(
        Path::new(".feature_generator_config.toml"),
        include_str!("./exp/feature_generator_config.toml"),
    );
}

#[test]
fn generate_library() {}

fn assert_content_eq(file_path: &Path, expected_contents: &str) {
    let file = File::open(file_path).unwrap();
    let contents = std::io::read_to_string(file);
    assert_eq!(expected_contents, contents.unwrap());
}

fn default_app_ctx() -> AppCtx {
    AppCtx {
        is_testing: false,
        app_name: "test-app".to_string(),
        base_package: "test.base.package".to_string(),
    }
}
