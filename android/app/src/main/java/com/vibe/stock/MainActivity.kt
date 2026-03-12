package com.vibe.stock

import android.content.Intent
import android.content.SharedPreferences
import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.vibe.stock.databinding.ActivityMainBinding
import kotlinx.coroutines.launch

/**
 * MainActivity — VibeStock Scanner entry point
 *
 * Presents a login form on first launch (or after logout).
 * On successful JWT login the token is saved to SharedPreferences
 * and the user is taken directly to the scanner screen.
 *
 * If a valid token already exists in SharedPreferences, the
 * login screen is skipped automatically.
 */
class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding
    private lateinit var prefs:   SharedPreferences

    companion object {
        const val PREFS_NAME  = "vibestock_prefs"
        const val KEY_TOKEN   = "jwt_token"
        const val KEY_USER    = "username"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        prefs = getSharedPreferences(PREFS_NAME, MODE_PRIVATE)

        // Auto-login if token exists
        val existingToken = prefs.getString(KEY_TOKEN, null)
        if (!existingToken.isNullOrBlank()) {
            openScanner()
            return
        }

        // ── Login button handler ───────────────────────────────────────────────
        binding.btnLogin.setOnClickListener {
            val username = binding.etUsername.text.toString().trim()
            val password = binding.etPassword.text.toString()

            if (username.isEmpty() || password.isEmpty()) {
                binding.tvError.text = "Please fill in all fields."
                binding.tvError.visibility = View.VISIBLE
                return@setOnClickListener
            }

            binding.btnLogin.isEnabled = false
            binding.progressBar.visibility = View.VISIBLE
            binding.tvError.visibility = View.GONE

            lifecycleScope.launch {
                try {
                    val resp = ApiClient.api.login(LoginRequest(username, password))

                    // Persist token for future launches
                    prefs.edit()
                        .putString(KEY_TOKEN, resp.token)
                        .putString(KEY_USER,  resp.full_name)
                        .apply()

                    openScanner()
                } catch (e: Exception) {
                    binding.tvError.text = "Login failed: ${e.message}"
                    binding.tvError.visibility = View.VISIBLE
                    binding.btnLogin.isEnabled  = true
                    binding.progressBar.visibility = View.GONE
                }
            }
        }
    }

    /** Launch the barcode scanner screen */
    private fun openScanner() {
        startActivity(Intent(this, ScannerActivity::class.java))
        finish()   // Remove login from back-stack
    }
}
