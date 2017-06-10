
import java.util.*


var tries: Long = 0

fun main(args: Array<String>) {
    println("Magic square search")
    val start = System.currentTimeMillis()

    Runtime.getRuntime().addShutdownHook(Thread({
        println("Finished")
        println("Count: " + tries)
        val timeSpend = (System.currentTimeMillis() - start) / 1000f
        println("Time spend " + timeSpend)
        println("Performance " + tries / timeSpend + " tries / s ")
    }))

    for (i in 0..Runtime.getRuntime().availableProcessors())
        Thread({findSquare()}).start()
//    findSquare()
}

private fun findSquare() {
    val square = Square(3, 100)
    val rand = Random()
    while (true) {
        tries += 1
        square.populate({ _, _ -> rand.nextInt(100) + 1 })
        powerOf2(square)
        if (isMagic(square)) {
            println("Magic square found!")
            println("Square: " + square)
            break
        }
    }
}

class Square(val size: Int = 3, val maxvalue: Int = 100) {
    val values: Array<IntArray> = Array(size, { IntArray(size) })


    fun populate(func: (Int, Int) -> (Int)) {
        for (row in values) {
            Arrays.fill(row, 0)
        }
        for (x in 0..size - 1) {
            for (y in 0..size - 1) {
                values[x][y] = checkAndGet(x, y, func)
            }
        }
    }

    fun checkAndGet(row: Int, column: Int, populate: (Int, Int) -> Int): Int {
        while (true) {
            val value = populate(row, column)
            if (!contains(value)) return value
        }
    }

    fun contains(newValue: Int): Boolean {
        for (row in values) {
            for (value in row) {
                if (value == newValue) return true
            }
        }
        return false
    }

    override fun toString(): String {
        return Arrays.toString(values.map { row -> Arrays.toString(row) }.toTypedArray())
    }
}

fun powerOf2(square: Square): Unit {
    for (x in 0..square.size - 1) {
        for (y in 0..square.size - 1) {
            val value = square.values[x][y]
            square.values[x][y] = value * value
        }
    }
}

fun isMagic(square: Square): Boolean {
    val defaultTotal = intArrayOf(-1, -1)
    var lastTotal = defaultTotal.clone()
    val diagonals = IntArray(2)
    val total = IntArray(2)

    for (x in 0..square.size - 1) {
        total.fill(0)
        for (y in 0..square.size - 1) {
            total[0] += square.values[x][y]
            total[1] += square.values[y][x]
            if (x == y) diagonals[0] += square.values[x][y]
            if (square.size - x - 1 == y) diagonals[1] += square.values[x][y]
        }
        if (!lastTotal.contentEquals(defaultTotal) && !lastTotal.contentEquals(total))
            return false
        lastTotal = total.clone()
    }
    if (total[0] == total[1] && total[0] == diagonals[0] && diagonals[0] == diagonals[1]) {
        println("Magic square sum " + total[0])
        return true
    } else return false
}