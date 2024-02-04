package com.example.sgs

import android.app.NativeActivity
//import android.os.Bundle
//import android.view.WindowManager

class MainActivity : NativeActivity() {
    companion object {
        init {
            System.loadLibrary("sgs")
        }
    }

    /*override protected fun onCreate(savedInstanceState: Bundle) {
        WindowCompat.setDecorFitsSystemWindows(getWindow(), true);
    }*/
}
