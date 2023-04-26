package com.cardinalblue.moviesearch.impl.search.di

import com.cardinalblue.moviesearch.impl.search.usecase.DoExample
import com.cardinalblue.moviesearch.impl.search.usecase.DoExampleUseCase
import com.cardinalblue.navigation.SubfeatureScoped
import dagger.Binds
import dagger.Module

/**
 * Add subfeature-wide available bindings here
 */
@Module
interface SearchModule {
    @Binds
    @SubfeatureScoped
    fun bindDoExampleUseCase(impl: DoExampleUseCase): DoExample
}