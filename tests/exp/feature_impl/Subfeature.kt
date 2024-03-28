package test.base.package.home.impl.subfeature.home

import androidx.compose.runtime.Composable
import com.cardinalblue.navigation.EmptyInput
import test.base.package.home.impl.subfeature.home.screen.HomeScreen
import test.base.package.home.impl.subfeature.home.screen.HomeScreenViewModel
import com.cardinalblue.navigation.Subfeature
import com.cardinalblue.skeleton.processor.DeclareSubfeature
import dagger.Module

@Module
interface HomeSubfeatureModule

@DeclareSubfeature(
    route = "home",
    input = EmptyInput::class,
    subfeatureModule = HomeSubfeatureModule::class,
)
object HomeSubfeature : Subfeature<HomeScreenViewModel> {
    @Composable
    override fun Screen(viewModel: HomeScreenViewModel) {
        HomeScreen(viewModel = viewModel)
    }
}
