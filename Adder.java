public class Adder {
    static {
        System.loadLibrary("add"); // Load native Rust library named "adder"
    }

    // Declare native method implemented in Rust
    public static native int add(int a, int b);
    
    public static native void print();

    public static void main(String[] args) {
        int result = add(2, 3);
        System.out.println("2 + 3 = " + result);
        print();
    }
}
