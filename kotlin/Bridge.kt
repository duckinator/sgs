package rs.tts

import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener

//import androidx.annotation.Keep

//@Keep
abstract class Bridge(backendId: Int): UtteranceProgressListener(), TextToSpeech.OnInitListener {
    public var backendId: Int

    init {
        this.backendId = backendId
    }

    abstract override fun onInit(status: Int)

    abstract override fun onStart(utteranceId: String)

    abstract override fun onStop(utteranceId: String, interrupted: Boolean)

    abstract override fun onDone(utteranceId: String)

    abstract override fun onError(utteranceId: String)

}
