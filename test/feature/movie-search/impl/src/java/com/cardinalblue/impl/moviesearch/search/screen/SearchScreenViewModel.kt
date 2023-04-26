package com.cardinalblue.moviesearch.impl.search.screen

import androidx.lifecycle.ViewModel
import com.cardinalblue.moviesearch.impl.search.usecase.SearchMovies
import javax.inject.Inject

class SearchScreenViewModel @Inject constructor(
    private val doExample: DoExample,
) : ViewModel() {
    fun onClick() {
        doExample("example")
    }
}