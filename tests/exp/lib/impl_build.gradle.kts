plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("com.google.devtools.ksp")

    `android-config`
}

android {
    namespace = "test.base.package.facedetection.impl"
}

dependencies {
    implementation(platform(), navigation())
    implementation(faceDetection.api())
    implementation(libs.coroutines, libs.timber)

    implementation(libs.dagger)
    ksp(libs.daggerCompiler)

    implementation(scaffold())
    ksp(scaffoldProcessor())
}
