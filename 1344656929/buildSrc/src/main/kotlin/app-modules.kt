import org.gradle.kotlin.dsl.DependencyHandlerScope

// ===== feature modules =====
val DependencyHandlerScope.camera get() = createProject(":feature:camera")
val DependencyHandlerScope.memeFaceFusion get() = createProject(":feature:meme-face-fusion")
val DependencyHandlerScope.memeSearch get() = createProject(":feature:meme-search")
val DependencyHandlerScope.memeGenerator get() = createProject(":feature:meme-generator")
val DependencyHandlerScope.faceInitializer get() = createProject(":feature:face-initializer")
val DependencyHandlerScope.memeDetails get() = createProject(":feature:meme-details")
val DependencyHandlerScope.landing get() = createProject(":feature:landing")

val DependencyHandlerScope.appState get() = createProject(":library:app-state")

val DependencyHandlerScope.memeGeneration get() = createProject(":library:meme-generation")
val DependencyHandlerScope.savedFaces get() = createProject(":library:saved-faces")
