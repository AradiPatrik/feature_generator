plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("com.google.devtools.ksp")

    `android-config`
}

android {
    namespace = "test.base.package.home.api"
    applyCompose()
}

dependencies {
    implementation(navigation())
    implementation(libs.compose)

    implementation(libs.moshi)
    ksp(libs.moshiCompiler)

    implementation(scaffold())
    ksp(scaffoldProcessor())
}
