package com.cardinalblue.fresh

import android.app.Application
import test.base.package.home.impl.root.HomeFeatureRoot
import androidx.lifecycle.ViewModel
import com.cardinalblue.AppEventTracker
import test.base.package.start.impl.entry.StartFeatureRoot
import com.cardinalblue.platform.EventTracker
import com.cardinalblue.platform.Platform
import com.cardinalblue.platform.PlatformLibraryConfig
import com.cardinalblue.platform.PlatformLibraryModule
import com.cardinalblue.skeleton.processor.Scaffold
import javax.inject.Inject

class AppViewModel @Inject constructor() : ViewModel() {

}

@Scaffold(
    libraries = [Platform::class],
    features = [
        HomeFeatureRoot::class,
        StartFeatureRoot::class,
    ],
    startFeature = HomeFeatureRoot::class,
    appViewModel = AppViewModel::class,
)
class FreshApp : Application() {
    lateinit var appProvider: AppProvider
        private set

    override fun onCreate() {
        super.onCreate()

        appProvider = DaggerAppGraphComponent.factory()
            .create(
                PlatformLibraryModule(
                    PlatformLibraryConfig(
                        this,
                        AppEventTracker
                    )
                )
            )
            .appProvider
    }
}

val Application.appProvider get() = (this as FreshApp).appProvider
