plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    kotlin("kapt")

    `android-config`
}

android {
    namespace = "com.cardinalblue.moviesearch.impl"
    applyCompose()
}

dependencies {
    applyFeatureCommon()
    implementation(movieSearch.api())
    implementation(libs.compose, libs.coroutines, libs.paging)

    implementation(libs.dagger)
    kapt(libs.daggerCompiler)
}