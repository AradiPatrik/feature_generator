package test.base.package.home.api

import com.cardinalblue.navigation.EmptyInput
import com.cardinalblue.navigation.FeatureEntry
import com.cardinalblue.navigation.NavDirection
import com.cardinalblue.navigation.createNavDirection

/**
 * Defines route and input for this feature
 */
interface HomeFeatureEntry : FeatureEntry {
    companion object :
        NavDirection<EmptyInput> by createNavDirection("home")
}
