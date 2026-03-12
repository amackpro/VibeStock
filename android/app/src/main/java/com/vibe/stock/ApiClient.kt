package com.vibe.stock

import com.squareup.moshi.Moshi
import com.squareup.moshi.kotlin.reflect.KotlinJsonAdapterFactory
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.moshi.MoshiConverterFactory
import retrofit2.http.*
import java.util.concurrent.TimeUnit

// =============================================================================
// Data Models — mirrors the Rust shared crate DTOs
// =============================================================================

/**
 * Product returned by the VibeStock API (with joined category/supplier names).
 */
data class Product(
    val id:                  String,
    val name:                String,
    val description:         String?,
    val sku:                 String,
    val barcode:             String?,
    val category_name:       String?,
    val supplier_name:       String?,
    val unit_price:          Double,
    val cost_price:          Double,
    val quantity_in_stock:   Int,
    val reorder_level:       Int,
    val unit_of_measure:     String,
    val is_active:           Boolean,
)

/**
 * Paginated product list response from GET /api/products
 */
data class ProductPage(
    val data:        List<Product>,
    val total:       Long,
    val page:        Long,
    val per_page:    Long,
    val total_pages: Long,
)

/**
 * JWT login request body
 */
data class LoginRequest(val username: String, val password: String)

/**
 * JWT login response returned by POST /api/auth/login
 */
data class LoginResponse(
    val token:     String,
    val user_id:   String,
    val username:  String,
    val full_name: String,
    val role:      String,
)

/**
 * Stock movement creation request body
 */
data class MovementRequest(
    val product_id:    String,
    val movement_type: String,   // "in" | "out" | "adjustment" | "return"
    val quantity:      Int,
    val reference:     String?,
    val notes:         String?,
)

/**
 * Stock movement creation response
 */
data class MovementResponse(
    val message:      String,
    val id:           String,
    val new_quantity: Int,
)

// =============================================================================
// Retrofit Service Interface
// =============================================================================

/**
 * VibeStockApi — Retrofit interface mapping to all Rust API endpoints.
 */
interface VibeStockApi {

    /** Authenticate and receive a JWT */
    @POST("/api/auth/login")
    suspend fun login(@Body body: LoginRequest): LoginResponse

    /**
     * Look up a product by its barcode (EAN-13 / QR).
     * Used as the primary action after the camera scans a barcode.
     */
    @GET("/api/products/barcode/{barcode}")
    suspend fun getProductByBarcode(
        @Header("Authorization") token: String,
        @Path("barcode") barcode: String,
    ): Product

    /** Search products by name or SKU */
    @GET("/api/products")
    suspend fun listProducts(
        @Header("Authorization") token: String,
        @Query("search")   search:  String = "",
        @Query("page")     page:    Int    = 1,
        @Query("per_page") perPage: Int    = 20,
    ): ProductPage

    /** Record a stock movement (in / out / adjustment / return) */
    @POST("/api/movements")
    suspend fun createMovement(
        @Header("Authorization") token: String,
        @Body body: MovementRequest,
    ): MovementResponse
}

// =============================================================================
// ApiClient singleton — initialises Retrofit with OkHttp + Moshi
// =============================================================================

/**
 * Singleton API client.
 *
 * Usage:
 *   val product = ApiClient.api.getProductByBarcode("Bearer $token", barcode)
 */
object ApiClient {

    private val moshi = Moshi.Builder()
        .addLast(KotlinJsonAdapterFactory())
        .build()

    private val okHttp = OkHttpClient.Builder()
        .connectTimeout(10, TimeUnit.SECONDS)
        .readTimeout(15, TimeUnit.SECONDS)
        .addInterceptor(HttpLoggingInterceptor().apply {
            level = HttpLoggingInterceptor.Level.BODY
        })
        .build()

    val api: VibeStockApi by lazy {
        Retrofit.Builder()
            .baseUrl(BuildConfig.API_BASE_URL)
            .addConverterFactory(MoshiConverterFactory.create(moshi))
            .client(okHttp)
            .build()
            .create(VibeStockApi::class.java)
    }
}
