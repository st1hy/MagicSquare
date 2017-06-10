import java.util.Arrays;
import java.util.Random;

public class MagicSquare {

    static volatile long tries = 0;

    public static void main(String... args) {
        long start = System.currentTimeMillis();
        println("Magic square");
        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            println("Finished");
            println("Count " + tries);
            float time = (System.currentTimeMillis() - start) / 1000f;
            println("Time stamp " + time + " s");
            println("Performance " + tries / time + " tries / s");
        }));

        int processors = Runtime.getRuntime().availableProcessors();
        for (int i = 0; i < processors; i++) {
            new Thread(MagicSquare::findSquare).start();
        }
//        findSquare();
    }

    private static void findSquare() {
        Square square = new Square(3, 10000);
        Random rand = new Random();
        while (true) {
            tries += 1;
            square.populate(((x, y) -> rand.nextInt(square.maxValue) + 1));
            powerOf2(square);
            if (isMagic(square)) {
                println("Magic square found");
                println(square.toString());
                break;
            }
        }
    }

    private static boolean isMagic(Square square) {
        int[] defaultTotal = {-1, -1};
        int[] lastTotal = defaultTotal.clone();
        int[] diagonal = {0, 0};
        int[] total = {0, 0};
        for (int x = 0; x < square.size; x++) {
            Arrays.fill(total, 0);
            for (int y = 0; y < square.size; y++) {
                total[0] += square.values[x][y];
                total[1] += square.values[y][x];
                if (x == y) diagonal[0] += square.values[x][y];
                if (square.size - x - 1 == y) diagonal[1] += square.values[x][y];
            }
            if (!Arrays.equals(lastTotal, defaultTotal) && !Arrays.equals(lastTotal, total))
                return false;
            lastTotal = total.clone();
        }
        if (total[0] == total[1] && total[0] == diagonal[0] && diagonal[0] == diagonal[1]) {
            println("Magic square sum " + total[0]);
            return true;
        } else return false;
    }

    private static void println(String x) {
        System.out.println(x);
    }

    private static void powerOf2(Square square) {
        for (int x = 0; x < square.size; x++) {
            for (int y = 0; y < square.size; y++) {
                int value = square.values[x][y];
                square.values[x][y] = value * value;
            }
        }
    }

    static class Square {
        final int size, maxValue;
        int[][] values;

        public Square(int size, int maxValue) {
            this.size = size;
            this.maxValue = maxValue;
            values = new int[size][size];
        }


        public void populate(Generator generator) {
            for (int[] row : values) {
                Arrays.fill(row, 0);
            }
            for (int x = 0; x < size; x++) {
                for (int y = 0; y < size; y++) {
                    values[x][y] = checkAndGenerate(generator, x, y);
                }
            }
        }

        private int checkAndGenerate(Generator generator, int x, int y) {
            while (true) {
                int value = generator.generate(x, y);
                if (!contains(value)) return value;
            }
        }

        private boolean contains(int newValue) {
            for (int[] row : values) {
                for (int value : row) {
                    if (value == newValue) return true;
                }
            }
            return false;
        }

        @Override
        public String toString() {
            StringBuilder builder = new StringBuilder(100);
            builder.append("[");
            for (int[] row : values) {
                builder.append("[");
                for (int value : row) {
                    builder.append(value).append(",");
                }
                builder.deleteCharAt(builder.length() - 1);
                builder.append("],");
            }
            builder.deleteCharAt(builder.length() - 1);
            builder.append("]");
            return builder.toString();
        }

        interface Generator {
            int generate(int x, int y);
        }
    }
}
