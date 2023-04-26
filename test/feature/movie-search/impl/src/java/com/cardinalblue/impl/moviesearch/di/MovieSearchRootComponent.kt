package com.cardinalblue.moviesearch.impl.di

import com.cardinalblue.data.api.DataProvider
import com.cardinalblue.api.MovieSearchProvider
import com.cardinalblue.moviesearch.impl.search.di.SearchSubcomponent
import com.cardinalblue.navigation.FeatureScoped
import com.cardinalblue.platform.PlatformProvider
import dagger.Component

/**
 * The root component for the feature. It is providing the feature-wide dependencies. Exposes them
 * to other features via [MovieSearchProvider].
 */
@FeatureScoped
@Component(
    dependencies = [
        DataProvider::class,
        PlatformProvider::class,
    ],
    modules = [MovieSearchRootModule::class, MovieSearchSubcomponentsModule::class]
)
interface MovieSearchRootComponent : MovieSearchProvider {
    val searchSubcomponentFactory: SearchSubcomponent.Factory
}
