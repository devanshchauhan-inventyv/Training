import java.util.Scanner;

public class maxOf4 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        int x, y, z,w;
        System.out.print("Enter value for x: ");
        x = scanner.nextInt();
        System.out.print("Enter value for y: ");
        y = scanner.nextInt();
        System.out.print("Enter value for z: ");
        z = scanner.nextInt();
        System.out.print("Enter value for w: ");
        w = scanner.nextInt();


        boolean condition1 = x >= y;
        boolean condition2 = y <= z;
        boolean condition3 = x >= z;
        boolean condition4 = y >= w;
        boolean condition5 = z >= w;
        boolean condition6 = x >= w;

        if (condition1) {
            if (condition2) {
                if (condition3) {
                    if (condition4) {
                        System.out.println(y);
                    }
                    if (condition5) {
                        System.out.println(w);
                    }
                    System.out.println(z);
                }
                if (condition5) {
                    System.out.println(x);
                }
                System.out.println(z);
            }
            if (condition6) {
                System.out.println(x);
            }
        }

        scanner.close();
    }
}
