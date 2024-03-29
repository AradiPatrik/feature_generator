package test.base.package.facedetection.impl.usecase

import android.content.Context
import test.base.package.myapp.facedetection.GetExample
import test.base.package.facedetection.impl.FaceDetection
import com.cardinalblue.platform.ApplicationContext
import com.cardinalblue.platform.IoDispatcher
import com.cardinalblue.skeleton.processor.InLibrary
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.withContext
import javax.inject.Inject

@InLibrary(FaceDetection::class)
class GetExampleUseCase @Inject constructor(
    @IoDispatcher private val ioDispatcher: CoroutineDispatcher,
    @ApplicationContext private val context: Context
) : GetExample {
    override suspend fun invoke() = withContext(ioDispatcher) {
        TODO("Implement this")
    }
}
