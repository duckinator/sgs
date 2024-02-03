package rs.tts

import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener

@androidx.annotation.Keep
class Bridge: UtteranceProgressListener(), TextToSpeech.OnInitListener {
    public int backendId

    fun Bridge(backendId: int) {
        this.backendId = backendId
    }

    override fun onInit(status: int)

    override fun onStart(utteranceId: String)

    override fun onStop(utteranceId: String, interrupted: Boolean)

    override fun onDone(utteranceId: String)

    override fun onError(utteranceId: String)

}
