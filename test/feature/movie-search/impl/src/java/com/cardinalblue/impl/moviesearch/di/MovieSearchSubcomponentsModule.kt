package com.cardinalblue.moviesearch.impl.di

import com.cardinalblue.moviesearch.impl.search.di.SearchSubcomponent
import dagger.Module

/**
 * Add subfeature-components here
 */
@Module(subcomponents = [SearchSubcomponent::class])
interface MovieSearchSubcomponentsModule