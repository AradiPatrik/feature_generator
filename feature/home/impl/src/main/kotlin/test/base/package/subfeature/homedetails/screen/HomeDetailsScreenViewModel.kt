package test.base.package.home.impl.subfeature.homedetails.screen

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.cardinalblue.navigation.AssistedViewModelFactory
import com.cardinalblue.navigation.EmptyInput
import kotlinx.coroutines.launch
import dagger.assisted.Assisted
import dagger.assisted.AssistedFactory
import dagger.assisted.AssistedInject

class HomeDetailsScreenViewModel @AssistedInject constructor(
    @Assisted private val savedStateHandle: SavedStateHandle,
    @Assisted private val input: EmptyInput,
) : ViewModel() {
    @AssistedFactory
    interface Factory : AssistedViewModelFactory<EmptyInput, HomeDetailsScreenViewModel>

    fun onClick() = viewModelScope.launch {
        TODO()
    }
}
