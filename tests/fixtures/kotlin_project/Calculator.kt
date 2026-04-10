package com.example.calculator

import kotlin.math.sqrt
import kotlin.math.pow

/**
 * A simple calculator class that performs basic arithmetic operations.
 */
class Calculator(private val precision: Int = 2) : MathOperations {

    private var lastResult: Double = 0.0

    /** Add two numbers */
    override fun add(a: Double, b: Double): Double {
        lastResult = a + b
        return lastResult
    }

    /** Subtract two numbers */
    override fun subtract(a: Double, b: Double): Double {
        lastResult = a - b
        return lastResult
    }

    /** Multiply two numbers */
    fun multiply(a: Double, b: Double): Double {
        lastResult = a * b
        return lastResult
    }

    /** Divide two numbers */
    fun divide(a: Double, b: Double): Double {
        require(b != 0.0) { "Cannot divide by zero" }
        lastResult = a / b
        return lastResult
    }

    fun getLastResult(): Double = lastResult
}

/**
 * Interface defining math operations
 */
interface MathOperations {
    fun add(a: Double, b: Double): Double
    fun subtract(a: Double, b: Double): Double
}

/**
 * Singleton object for advanced math utilities
 */
object MathUtils {
    fun squareRoot(x: Double): Double = sqrt(x)
    fun power(base: Double, exponent: Double): Double = base.pow(exponent)
}

/** Top-level helper function */
fun formatResult(value: Double, decimals: Int = 2): String {
    return "%.${decimals}f".format(value)
}

/**
 * Data class representing a calculation result
 */
data class CalculationResult(
    val operation: String,
    val operands: List<Double>,
    val result: Double
)

enum class Operation {
    ADD, SUBTRACT, MULTIPLY, DIVIDE
}
