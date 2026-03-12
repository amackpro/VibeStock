package com.vibestock.scanner

import android.Manifest
import android.content.SharedPreferences
import android.content.pm.PackageManager
import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.camera.core.*
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.lifecycle.lifecycleScope
import com.google.mlkit.vision.barcode.BarcodeScannerOptions
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.barcode.common.Barcode
import com.google.mlkit.vision.common.InputImage
import com.vibestock.scanner.databinding.ActivityScannerBinding
import kotlinx.coroutines.launch
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

/**
 * ScannerActivity — Barcode scanning + stock update screen
 *
 * Flow:
 *  1. Camera preview with ML Kit real-time barcode detection.
 *  2. On barcode detected → call GET /api/products/barcode/{code}.
 *  3. Display product name, SKU, current stock, unit price.
 *  4. User selects movement type + quantity → POST /api/movements.
 *  5. New stock quantity is shown with a success confirmation.
 */
class ScannerActivity : AppCompatActivity() {

    private lateinit var binding:       ActivityScannerBinding
    private lateinit var prefs:         SharedPreferences
    private lateinit var cameraExecutor: ExecutorService

    private var scannedProduct: Product? = null
    private var isScanning = true        // Pause scanning after first hit

    companion object {
        private const val CAMERA_PERMISSION_CODE = 1001
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityScannerBinding.inflate(layoutInflater)
        setContentView(binding.root)
        prefs = getSharedPreferences(MainActivity.PREFS_NAME, MODE_PRIVATE)

        supportActionBar?.title = "VibeStock Scanner"

        cameraExecutor = Executors.newSingleThreadExecutor()

        // Request camera permission
        if (ContextCompat.checkSelfPermission(this, Manifest.permission.CAMERA)
            == PackageManager.PERMISSION_GRANTED
        ) {
            startCamera()
        } else {
            ActivityCompat.requestPermissions(
                this,
                arrayOf(Manifest.permission.CAMERA),
                CAMERA_PERMISSION_CODE
            )
        }

        setupUi()
    }

    // ── Camera ───────────────────────────────────────────────────────────────

    private fun startCamera() {
        val cameraProviderFuture = ProcessCameraProvider.getInstance(this)
        cameraProviderFuture.addListener({
            val cameraProvider = cameraProviderFuture.get()

            // Preview use case
            val preview = Preview.Builder().build().also {
                it.setSurfaceProvider(binding.cameraPreview.surfaceProvider)
            }

            // Image analysis use case for ML Kit
            val imageAnalysis = ImageAnalysis.Builder()
                .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
                .build()
                .also { analysis ->
                    analysis.setAnalyzer(cameraExecutor, BarcodeAnalyzer { barcode ->
                        if (isScanning) {
                            isScanning = false
                            runOnUiThread { handleBarcode(barcode) }
                        }
                    })
                }

            try {
                cameraProvider.unbindAll()
                cameraProvider.bindToLifecycle(
                    this,
                    CameraSelector.DEFAULT_BACK_CAMERA,
                    preview,
                    imageAnalysis
                )
            } catch (e: Exception) {
                showError("Camera error: ${e.message}")
            }
        }, ContextCompat.getMainExecutor(this))
    }

    // ── Barcode → API lookup ──────────────────────────────────────────────────

    private fun handleBarcode(barcodeValue: String) {
        binding.tvStatus.text = "Scanning: $barcodeValue…"
        binding.progressBar.visibility = View.VISIBLE
        binding.cardProduct.visibility = View.GONE

        val token = "Bearer ${prefs.getString(MainActivity.KEY_TOKEN, "")}"

        lifecycleScope.launch {
            try {
                val product = ApiClient.api.getProductByBarcode(token, barcodeValue)
                scannedProduct = product
                showProductCard(product)
            } catch (e: Exception) {
                showError("Product not found for barcode: $barcodeValue")
                // Allow re-scan after error
                isScanning = true
            } finally {
                binding.progressBar.visibility = View.GONE
            }
        }
    }

    // ── UI helpers ────────────────────────────────────────────────────────────

    private fun showProductCard(p: Product) {
        binding.tvStatus.text = "Product found"
        binding.cardProduct.visibility = View.VISIBLE

        binding.tvProductName.text  = p.name
        binding.tvSku.text          = "SKU: ${p.sku}"
        binding.tvCategory.text     = p.category_name ?: "Uncategorised"
        binding.tvCurrentStock.text = "Stock: ${p.quantity_in_stock} ${p.unit_of_measure}"
        binding.tvPrice.text        = "₹%.2f".format(p.unit_price)

        // Stock status color
        val color = when {
            p.quantity_in_stock == 0              -> getColor(android.R.color.holo_red_light)
            p.quantity_in_stock <= p.reorder_level -> getColor(android.R.color.holo_orange_light)
            else                                   -> getColor(android.R.color.holo_green_light)
        }
        binding.tvCurrentStock.setTextColor(color)
    }

    private fun showError(msg: String) {
        binding.tvStatus.text = "⚠️ $msg"
        binding.cardProduct.visibility = View.GONE
    }

    private fun setupUi() {
        // Record movement button
        binding.btnRecordMovement.setOnClickListener {
            val p = scannedProduct ?: return@setOnClickListener
            val qty = binding.etQuantity.text.toString().toIntOrNull() ?: return@setOnClickListener
            val type = when (binding.rgMovementType.checkedRadioButtonId) {
                binding.rbIn.id         -> "in"
                binding.rbOut.id        -> "out"
                binding.rbAdjust.id     -> "adjustment"
                else                    -> "in"
            }

            recordMovement(p, type, qty)
        }

        // Scan again button — re-enable scanning
        binding.btnScanAgain.setOnClickListener {
            scannedProduct = null
            isScanning = true
            binding.cardProduct.visibility = View.GONE
            binding.tvStatus.text = "Point camera at barcode…"
        }

        // Logout
        binding.btnLogout.setOnClickListener {
            prefs.edit().remove(MainActivity.KEY_TOKEN).apply()
            startActivity(android.content.Intent(this, MainActivity::class.java))
            finish()
        }

        binding.tvStatus.text = "Point camera at a barcode…"
    }

    // ── Stock movement API call ───────────────────────────────────────────────

    private fun recordMovement(product: Product, type: String, quantity: Int) {
        binding.btnRecordMovement.isEnabled = false
        binding.progressBar.visibility = View.VISIBLE

        val token = "Bearer ${prefs.getString(MainActivity.KEY_TOKEN, "")}"

        lifecycleScope.launch {
            try {
                val resp = ApiClient.api.createMovement(
                    token,
                    MovementRequest(
                        product_id    = product.id,
                        movement_type = type,
                        quantity      = quantity,
                        reference     = null,
                        notes         = "Scanned via VibeStock Android app",
                    )
                )
                binding.tvStatus.text = "✅ Done! New stock: ${resp.new_quantity} ${product.unit_of_measure}"
                binding.tvCurrentStock.text = "Stock: ${resp.new_quantity} ${product.unit_of_measure}"
                // Update local product state
                scannedProduct = product.copy(quantity_in_stock = resp.new_quantity)
            } catch (e: Exception) {
                showError(e.message ?: "Failed to record movement")
            } finally {
                binding.progressBar.visibility = View.GONE
                binding.btnRecordMovement.isEnabled = true
            }
        }
    }

    override fun onRequestPermissionsResult(code: Int, perms: Array<out String>, results: IntArray) {
        super.onRequestPermissionsResult(code, perms, results)
        if (code == CAMERA_PERMISSION_CODE && results.firstOrNull() == PackageManager.PERMISSION_GRANTED) {
            startCamera()
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        cameraExecutor.shutdown()
    }
}

// =============================================================================
// ML Kit Barcode Analyzer — ImageAnalysis.Analyzer implementation
// =============================================================================

/**
 * Processes each camera frame through ML Kit barcode scanner.
 * Calls [onBarcodeDetected] on the first valid barcode found per frame.
 */
@androidx.camera.core.ExperimentalGetImage
class BarcodeAnalyzer(private val onBarcodeDetected: (String) -> Unit) : ImageAnalysis.Analyzer {

    private val options = BarcodeScannerOptions.Builder()
        .setBarcodeFormats(
            Barcode.FORMAT_EAN_13,
            Barcode.FORMAT_EAN_8,
            Barcode.FORMAT_CODE_128,
            Barcode.FORMAT_QR_CODE,
            Barcode.FORMAT_UPC_A,
            Barcode.FORMAT_UPC_E,
        )
        .build()

    private val scanner = BarcodeScanning.getClient(options)

    override fun analyze(imageProxy: ImageProxy) {
        val mediaImage = imageProxy.image ?: run { imageProxy.close(); return }
        val image = InputImage.fromMediaImage(mediaImage, imageProxy.imageInfo.rotationDegrees)

        scanner.process(image)
            .addOnSuccessListener { barcodes ->
                barcodes.firstOrNull()?.rawValue?.let { onBarcodeDetected(it) }
            }
            .addOnCompleteListener { imageProxy.close() }
    }
}
