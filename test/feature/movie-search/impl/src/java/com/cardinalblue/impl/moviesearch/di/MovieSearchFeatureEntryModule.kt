package com.cardinalblue.moviesearch.impl.di

import com.cardinalblue.moviesearch.api.MovieSearchFeatureEntry
import com.cardinalblue.moviesearch.impl.entry.MovieSearchFeatureEntryImpl
import com.cardinalblue.navigation.FeatureEntry
import com.cardinalblue.navigation.FeatureEntryKey
import dagger.Binds
import dagger.Module
import dagger.multibindings.IntoMap
import javax.inject.Singleton

/**
 * Binding the navigation entry points into this feature
 */
@Module
interface MovieSearchFeatureEntryModule {
    @Binds
    @IntoMap
    @FeatureEntryKey(MovieSearchFeatureEntry::class)
    @Singleton
    fun bindMovieSearchEntry(impl: MovieSearchFeatureEntryImpl): FeatureEntry<*>
}