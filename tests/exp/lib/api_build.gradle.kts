plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")

    `android-config`
}

android {
    namespace = "test.base.package.facedetection.api"
}

dependencies {
    implementation(libs.coroutines)
}
