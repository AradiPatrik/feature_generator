package test.base.package.home.impl.subfeature.home.screen

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.cardinalblue.navigation.AssistedViewModelFactory
import com.cardinalblue.navigation.EmptyInput
import kotlinx.coroutines.launch
import dagger.assisted.Assisted
import dagger.assisted.AssistedFactory
import dagger.assisted.AssistedInject

class HomeScreenViewModel @AssistedInject constructor(
    @Assisted private val savedStateHandle: SavedStateHandle,
    @Assisted private val input: EmptyInput,
) : ViewModel() {
    @AssistedFactory
    interface Factory : AssistedViewModelFactory<EmptyInput, HomeScreenViewModel>

    fun onClick() = viewModelScope.launch {
        TODO()
    }
}
