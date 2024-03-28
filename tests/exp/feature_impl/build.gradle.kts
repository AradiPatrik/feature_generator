plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("com.google.devtools.ksp")

    `android-config`
}

android {
    namespace = "test.base.package.home.impl"
    applyCompose()
}

dependencies {
    applyFeatureCommon()
    implementation(home.api())
    implementation(libs.compose, libs.coroutines)

    implementation(libs.dagger)
    ksp(libs.daggerCompiler)

    implementation(libs.moshi)
    ksp(libs.moshiCompiler)

    implementation(scaffold())
    ksp(scaffoldProcessor())
}
