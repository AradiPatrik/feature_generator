package test.base.package.home.impl.root

import test.base.package.home.api.HomeFeatureEntry
import test.base.package.home.impl.subfeature.home.HomeSubfeature
import com.cardinalblue.platform.PlatformProvider
import com.cardinalblue.skeleton.processor.FeatureRoot
import dagger.Module

@Module
interface HomeRootModule

@FeatureRoot(
    dependencies = [
        PlatformProvider::class,
    ],
    rootModule = HomeRootModule::class,
    startSubfeature = HomeSubfeature::class,
    featureEntry = HomeFeatureEntry::class,
)
interface HomeFeatureRoot
