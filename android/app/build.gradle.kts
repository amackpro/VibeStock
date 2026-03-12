/**
 * app/build.gradle.kts — VibeStock Scanner module
 *
 * Dependencies:
 *  - CameraX      : live camera preview for barcode scanning
 *  - ML Kit       : on-device barcode detection (no network needed)
 *  - Retrofit 2   : REST client calling the VibeStock Rust API
 *  - Moshi        : JSON serialization
 *  - Coroutines   : async API calls
 *  - Material 3   : modern UI components
 */
plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
}

android {
    namespace   = "com.vibestock.scanner"
    compileSdk  = 34

    defaultConfig {
        applicationId = "com.vibestock.scanner"
        minSdk        = 26
        targetSdk     = 34
        versionCode   = 1
        versionName   = "1.0.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"

        // API base URL — 10.0.2.2 accesses localhost on the Android Emulator
        // Change to your machine's LAN IP (e.g., 192.168.1.x) if using a physical device
        buildConfigField("String", "API_BASE_URL", "\"http://10.0.2.2:3000\"")
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }

    buildFeatures {
        buildConfig = true
        viewBinding = true
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
}

dependencies {
    // ── AndroidX Core ─────────────────────────────────────────────────────────
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.appcompat)
    implementation(libs.material)
    implementation(libs.androidx.activity)
    implementation(libs.androidx.constraintlayout)
    implementation(libs.androidx.lifecycle.runtime.ktx)

    // ── CameraX ───────────────────────────────────────────────────────────────
    val cameraxVersion = "1.3.2"
    implementation("androidx.camera:camera-core:$cameraxVersion")
    implementation("androidx.camera:camera-camera2:$cameraxVersion")
    implementation("androidx.camera:camera-lifecycle:$cameraxVersion")
    implementation("androidx.camera:camera-view:$cameraxVersion")

    // ── ML Kit Barcode Scanning ───────────────────────────────────────────────
    implementation("com.google.mlkit:barcode-scanning:17.2.0")

    // ── Networking ────────────────────────────────────────────────────────────
    implementation("com.squareup.retrofit2:retrofit:2.9.0")
    implementation("com.squareup.retrofit2:converter-moshi:2.9.0")
    implementation("com.squareup.moshi:moshi-kotlin:1.15.1")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.okhttp3:logging-interceptor:4.12.0")

    // ── Coroutines ────────────────────────────────────────────────────────────
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3")
    implementation("androidx.lifecycle:lifecycle-viewmodel-ktx:2.7.0")
    implementation(libs.androidx.lifecycle.runtime.ktx)

    // ── Tests ─────────────────────────────────────────────────────────────────
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
}
