package test.base.package.home.impl.subfeature.homedetails

import androidx.compose.runtime.Composable
import com.cardinalblue.navigation.EmptyInput
import test.base.package.home.impl.subfeature.homedetails.screen.HomeDetailsScreen
import test.base.package.home.impl.subfeature.homedetails.screen.HomeDetailsScreenViewModel
import com.cardinalblue.navigation.Subfeature
import com.cardinalblue.skeleton.processor.DeclareSubfeature
import dagger.Module

@Module
interface HomeDetailsSubfeatureModule

@DeclareSubfeature(
    route = "home-details",
    input = EmptyInput::class,
    subfeatureModule = HomeDetailsSubfeatureModule::class,
)
object HomeDetailsSubfeature : Subfeature<HomeDetailsScreenViewModel> {
    @Composable
    override fun Screen(viewModel: HomeDetailsScreenViewModel) {
        HomeDetailsScreen(viewModel = viewModel)
    }
}
