package net.duckinator.sgs

import android.app.NativeActivity

class MainActivity : NativeActivity() {
    companion object {
        init {
            System.loadLibrary("sgs")
        }
    }
}
