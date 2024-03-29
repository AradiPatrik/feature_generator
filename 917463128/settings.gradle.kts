pluginManagement {
    repositories {
        google {
            content {
                includeGroupByRegex("com\\.android.*")
                includeGroupByRegex("com\\.google.*")
                includeGroupByRegex("androidx.*")
            }
        }
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "Fresh"
include(":app")

include(":core")

include(":core:navigation")
include(":core:platform")
include(":core:scaffold")
include(":core:scaffold:processor")
include(":core:data")
include(":feature:start:api")
include(":feature:start:impl")
include(":theme")
