package test.base.package.home.impl.subfeature.homedetails.screen

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import test.base.package.theme.TestAppTheme

@Composable
fun HomeDetailsScreen(viewModel: HomeDetailsScreenViewModel) {
    HomeDetailsScreen()
}

@Composable
fun HomeDetailsScreen() {
    Box(modifier = Modifier.fillMaxSize()) {
        Column(Modifier.fillMaxSize()) {
            Text(
                modifier = Modifier
                    .align(Alignment.CenterHorizontally)
                    .padding(16.dp),
                text = "HomeDetails Screen",
                style = MaterialTheme.typography.titleLarge
            )
        }
    }
}

@Composable
@Preview(showBackground = true)
fun HomeDetailsScreenPreview() {
    TestAppTheme {
        HomeDetailsScreen()
    }
}
