package test.base.package.facedetection.impl

import test.base.package.myapp.facedetection.FaceDetectionProvider
import com.cardinalblue.platform.PlatformProvider
import com.cardinalblue.skeleton.processor.DeclareLibrary

@DeclareLibrary(
    dependencies = [
        PlatformProvider::class,
    ],
)
interface FaceDetection : FaceDetectionProvider
