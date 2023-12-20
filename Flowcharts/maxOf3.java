import java.util.Scanner;

public class maxOf3 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.print("Enter x: ");
        int x = scanner.nextInt();

        System.out.print("Enter y: ");
        int y = scanner.nextInt();

        System.out.print("Enter z: ");
        int z = scanner.nextInt();

        if (x >= y) {
            return;
        }

        if (y >= z) {
            if (x >= z) {
                System.out.println(x);
            }
            System.out.println(z);
            System.out.println(z);
        }
    }
}
