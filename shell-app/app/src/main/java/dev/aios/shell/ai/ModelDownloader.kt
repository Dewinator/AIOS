// Copyright 2026 AIOS Contributors
// SPDX-License-Identifier: Apache-2.0

package dev.aios.shell.ai

import android.content.Context
import android.util.Log
import java.io.File
import java.io.FileOutputStream
import java.net.HttpURLConnection
import java.net.URL

/**
 * Verfuegbare GGUF-Modelle mit HuggingFace-Download-URLs.
 */
data class DownloadableModel(
    val id: String,
    val name: String,
    val fileName: String,
    val sizeBytes: Long,
    val sizeLabel: String,
    val url: String,
    val description: String,
)

/**
 * Download-Status fuer die UI.
 */
sealed class DownloadState {
    data object Idle : DownloadState()
    data class Downloading(val progress: Float, val downloadedBytes: Long, val totalBytes: Long) : DownloadState()
    data class Completed(val modelFile: File) : DownloadState()
    data class Failed(val error: String) : DownloadState()
}

/**
 * Laedt GGUF-Modelle von HuggingFace herunter und verwaltet lokale Modelle.
 *
 * Modelle werden im externen App-Verzeichnis unter /models/ gespeichert.
 * Der Nutzer kann Modelle bei Bedarf herunterladen und loeschen.
 */
class ModelDownloader(private val context: Context) {

    companion object {
        private const val TAG = "ModelDownloader"
        private const val MODELS_DIR = "models"
        private const val BUFFER_SIZE = 8192

        /**
         * Verfuegbare Modelle zum Herunterladen.
         * Kleine, quantisierte Modelle die auf Mobilgeraeten laufen.
         */
        val AVAILABLE_MODELS = listOf(
            DownloadableModel(
                id = "qwen2.5-1.5b-q4",
                name = "Qwen 2.5 1.5B (Q4_K_M)",
                fileName = "qwen2.5-1.5b-instruct-q4_k_m.gguf",
                sizeBytes = 1_100_000_000L,
                sizeLabel = "~1.1 GB",
                url = "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf",
                description = "Kompaktes Modell, gute Balance aus Geschwindigkeit und Qualitaet",
            ),
            DownloadableModel(
                id = "qwen2.5-3b-q4",
                name = "Qwen 2.5 3B (Q4_K_M)",
                fileName = "qwen2.5-3b-instruct-q4_k_m.gguf",
                sizeBytes = 2_100_000_000L,
                sizeLabel = "~2.1 GB",
                url = "https://huggingface.co/Qwen/Qwen2.5-3B-Instruct-GGUF/resolve/main/qwen2.5-3b-instruct-q4_k_m.gguf",
                description = "Empfohlen — beste Qualitaet fuer Mobilgeraete",
            ),
            DownloadableModel(
                id = "smollm2-1.7b-q4",
                name = "SmolLM2 1.7B (Q4_K_M)",
                fileName = "smollm2-1.7b-instruct-q4_k_m.gguf",
                sizeBytes = 1_000_000_000L,
                sizeLabel = "~1.0 GB",
                url = "https://huggingface.co/HuggingFaceTB/SmolLM2-1.7B-Instruct-GGUF/resolve/main/smollm2-1.7b-instruct-q4_k_m.gguf",
                description = "Sehr schnell, gut fuer einfache Aufgaben",
            ),
            DownloadableModel(
                id = "tinyllama-1.1b-q8",
                name = "TinyLlama 1.1B (Q8_0)",
                fileName = "tinyllama-1.1b-chat-v1.0.Q8_0.gguf",
                sizeBytes = 1_200_000_000L,
                sizeLabel = "~1.2 GB",
                url = "https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q8_0.gguf",
                description = "Winziges Modell, sehr schnelle Antworten",
            ),
        )
    }

    private val modelsDir: File
        get() {
            val dir = File(context.getExternalFilesDir(null), MODELS_DIR)
            if (!dir.exists()) dir.mkdirs()
            return dir
        }

    @Volatile
    private var cancelRequested = false

    /**
     * Gibt eine Liste der lokal vorhandenen Modelle zurueck.
     */
    fun getDownloadedModels(): List<DownloadableModel> {
        val localFiles = modelsDir.listFiles { file ->
            file.extension.equals("gguf", ignoreCase = true)
        } ?: return emptyList()

        return AVAILABLE_MODELS.filter { model ->
            localFiles.any { it.name == model.fileName }
        }
    }

    /**
     * Prueft ob ein bestimmtes Modell bereits heruntergeladen wurde.
     */
    fun isModelDownloaded(model: DownloadableModel): Boolean {
        val file = File(modelsDir, model.fileName)
        return file.exists() && file.length() > 0
    }

    /**
     * Gibt die Dateigroesse eines heruntergeladenen Modells zurueck.
     */
    fun getModelFileSize(model: DownloadableModel): Long {
        val file = File(modelsDir, model.fileName)
        return if (file.exists()) file.length() else 0
    }

    /**
     * Laedt ein Modell von HuggingFace herunter.
     * Muss auf einem Hintergrund-Thread aufgerufen werden.
     *
     * @param model Das herunterzuladende Modell.
     * @param onProgress Callback fuer Fortschrittsupdates.
     * @return DownloadState.Completed bei Erfolg, DownloadState.Failed bei Fehler.
     */
    fun downloadModel(
        model: DownloadableModel,
        onProgress: (DownloadState) -> Unit,
    ): DownloadState {
        cancelRequested = false
        val targetFile = File(modelsDir, model.fileName)
        val tempFile = File(modelsDir, "${model.fileName}.tmp")

        Log.i(TAG, "Starte Download: ${model.name} von ${model.url}")
        Log.i(TAG, "Ziel: ${targetFile.absolutePath}")

        onProgress(DownloadState.Downloading(0f, 0, model.sizeBytes))

        var connection: HttpURLConnection? = null
        try {
            // Unterstuetzung fuer Resume falls Temp-Datei existiert
            var downloadedBytes = if (tempFile.exists()) tempFile.length() else 0L

            connection = URL(model.url).openConnection() as HttpURLConnection
            connection.connectTimeout = 30_000
            connection.readTimeout = 30_000
            connection.instanceFollowRedirects = true

            if (downloadedBytes > 0) {
                connection.setRequestProperty("Range", "bytes=$downloadedBytes-")
            }

            val responseCode = connection.responseCode

            // Handle redirects (HuggingFace uses CDN redirects)
            if (responseCode == HttpURLConnection.HTTP_MOVED_TEMP ||
                responseCode == HttpURLConnection.HTTP_MOVED_PERM ||
                responseCode == 307 || responseCode == 308
            ) {
                val redirectUrl = connection.getHeaderField("Location")
                connection.disconnect()
                connection = URL(redirectUrl).openConnection() as HttpURLConnection
                connection.connectTimeout = 30_000
                connection.readTimeout = 30_000
                if (downloadedBytes > 0) {
                    connection.setRequestProperty("Range", "bytes=$downloadedBytes-")
                }
            }

            val finalResponseCode = connection.responseCode

            if (finalResponseCode != HttpURLConnection.HTTP_OK &&
                finalResponseCode != HttpURLConnection.HTTP_PARTIAL
            ) {
                val error = "HTTP $finalResponseCode: ${connection.responseMessage}"
                Log.e(TAG, "Download fehlgeschlagen: $error")
                return DownloadState.Failed(error)
            }

            val totalBytes = if (finalResponseCode == HttpURLConnection.HTTP_PARTIAL) {
                val contentRange = connection.getHeaderField("Content-Range")
                contentRange?.substringAfter("/")?.toLongOrNull() ?: (connection.contentLengthLong + downloadedBytes)
            } else {
                downloadedBytes = 0L // Server unterstuetzt kein Resume
                connection.contentLengthLong.let { if (it > 0) it else model.sizeBytes }
            }

            Log.i(TAG, "Download gestartet. Gesamt: ${totalBytes / 1024 / 1024} MB, bereits: ${downloadedBytes / 1024 / 1024} MB")

            val append = downloadedBytes > 0 && finalResponseCode == HttpURLConnection.HTTP_PARTIAL
            val outputStream = FileOutputStream(tempFile, append)
            val inputStream = connection.inputStream
            val buffer = ByteArray(BUFFER_SIZE)

            inputStream.use { input ->
                outputStream.use { output ->
                    var bytesRead: Int
                    var lastProgressUpdate = System.currentTimeMillis()

                    while (input.read(buffer).also { bytesRead = it } != -1) {
                        if (cancelRequested) {
                            Log.i(TAG, "Download abgebrochen")
                            return DownloadState.Failed("Download abgebrochen")
                        }

                        output.write(buffer, 0, bytesRead)
                        downloadedBytes += bytesRead

                        // Fortschritt alle 200ms aktualisieren
                        val now = System.currentTimeMillis()
                        if (now - lastProgressUpdate > 200) {
                            val progress = downloadedBytes.toFloat() / totalBytes.toFloat()
                            onProgress(DownloadState.Downloading(progress.coerceIn(0f, 1f), downloadedBytes, totalBytes))
                            lastProgressUpdate = now
                        }
                    }
                }
            }

            // Download abgeschlossen — Temp-Datei umbenennen
            if (targetFile.exists()) targetFile.delete()
            tempFile.renameTo(targetFile)

            Log.i(TAG, "Download abgeschlossen: ${targetFile.name} (${targetFile.length() / 1024 / 1024} MB)")
            val result = DownloadState.Completed(targetFile)
            onProgress(result)
            return result

        } catch (e: Exception) {
            Log.e(TAG, "Download-Fehler: ${e.message}", e)
            val error = e.message ?: "Unbekannter Fehler"
            val result = DownloadState.Failed(error)
            onProgress(result)
            return result
        } finally {
            connection?.disconnect()
        }
    }

    /**
     * Bricht einen laufenden Download ab.
     */
    fun cancelDownload() {
        cancelRequested = true
    }

    /**
     * Loescht ein heruntergeladenes Modell.
     */
    fun deleteModel(model: DownloadableModel): Boolean {
        val file = File(modelsDir, model.fileName)
        val tempFile = File(modelsDir, "${model.fileName}.tmp")
        tempFile.delete()
        return if (file.exists()) {
            val deleted = file.delete()
            Log.i(TAG, "Modell geloescht: ${model.fileName} — $deleted")
            deleted
        } else {
            true
        }
    }

    /**
     * Gibt den Pfad zum Modellverzeichnis zurueck.
     */
    fun getModelsDirectory(): File = modelsDir
}
