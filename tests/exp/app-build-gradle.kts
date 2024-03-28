plugins {
    id("com.android.application")
    id("kotlin-android")
//    id("com.google.gms.google-services")

    id("com.google.devtools.ksp")
//    id("com.google.firebase.crashlytics")

    // Precompiled plugin with the base android configuration.
    // Declared in buildSrc/.../android-config.gradle.kts.
    `android-config`
}

android {
    namespace = "com.cardinalblue.fresh"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.cardinalblue.fresh"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        vectorDrawables {
            useSupportLibrary = true
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = JavaVersion.VERSION_17.toString()
    }
    buildFeatures {
        compose = true
    }
    composeOptions {
        kotlinCompilerExtensionVersion = versions.composeCompiler
    }
    packaging {
        resources {
            excludes += "/META-INF/{AL2.0,LGPL2.1}"
        }
    }
}

dependencies {
    implementation(*home.all())
    implementation(libs.android)

    implementation(libs.dagger)
    ksp(libs.daggerCompiler)

    implementation(libs.glide)
    ksp(libs.glideAnnotationProcessor)

    implementation(libs.compose)

    implementation(scaffold())
    ksp(scaffoldProcessor())

    implementation(libs.timber)

    implementation(libs.coroutines)

    implementation(*start.all())

    implementation(
        platform(),
        data(),
        theme(),
        navigation()
    )
}
